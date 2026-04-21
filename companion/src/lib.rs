use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use notification::show_update_notification;
use serde::Deserialize;

mod http;
mod notification;

static UPDATE_ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Debug, Deserialize)]
struct RemoteFilesIndex {
    version: u32,
    files: BTreeMap<String, String>,
}

#[aviutl2::plugin(GenericPlugin)]
struct TranslationCompanion {}

impl aviutl2::generic::GenericPlugin for TranslationCompanion {
    fn new(_info: aviutl2::common::AviUtl2Info) -> aviutl2::common::AnyResult<Self> {
        init_tracing();
        Ok(Self {})
    }

    fn plugin_info(&self) -> aviutl2::generic::GenericPluginTable {
        aviutl2::generic::GenericPluginTable {
            name: "AviUtl2 Community Translation Companion".to_string(),
            information: "https://github.com/aviutl2/aviutl2-community-translation".to_string(),
        }
    }

    fn register(&mut self, _registry: &mut aviutl2::generic::HostAppHandle) {}

    fn on_project_load(&mut self, _project: &mut aviutl2::generic::ProjectFile) {
        UPDATE_ONCE.call_once(|| {
            std::thread::spawn(|| {
                if let Err(err) = run_update_task() {
                    tracing::warn!(?err, "companion update failed");
                }
            });
        });
    }
}

aviutl2::register_generic_plugin!(TranslationCompanion);

fn init_tracing() {
    let _ = aviutl2::tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_ansi(false)
        .event_format(aviutl2::logger::AviUtl2Formatter)
        .with_writer(aviutl2::logger::AviUtl2LogWriter)
        .try_init();
}

fn run_update_task() -> anyhow::Result<()> {
    let language_dir = aviutl2::config::app_data_path().join("Language");
    fs::create_dir_all(&language_dir)?;
    tracing::info!("checking for translation updates...");

    let index = fetch_remote_index()?;
    if index.version != 1 {
        tracing::warn!(version = index.version, "unsupported files index version");
        return Ok(());
    }

    let mut changed = false;
    for (file_name, remote_hash) in &index.files {
        tracing::debug!(file_name, remote_hash, "checking locale file");
        if !file_name.ends_with(".aul2") {
            continue;
        }

        let local_path = language_dir.join(file_name);
        let local_hash = hash_file_hex(&local_path).ok();
        if local_hash.as_deref() == Some(remote_hash.as_str()) {
            tracing::debug!(file_name, "locale file is up to date");
            continue;
        }

        let url = http::locale_file_url(file_name);
        let file_bytes = http::fetch_url_bytes(&url)?;
        let fetched_hash = hash_bytes_hex(&file_bytes);
        if fetched_hash != *remote_hash {
            tracing::warn!(
                file_name,
                expected_hash = remote_hash,
                actual_hash = fetched_hash,
                "hash mismatch for locale file"
            );
            continue;
        }

        tracing::info!(file_name, "updating locale file");
        write_atomic(&local_path, &file_bytes)?;
        changed = true;
    }

    if copy_language_overlay_files_if_needed(&language_dir)? {
        changed = true;
    }

    if changed
        && let Err(err) = show_update_notification(
            "AviUtl2 Community Translation",
            &aviutl2::config::get_language_text(
                "AviUtl2 Community Translation",
                "翻訳ファイルが更新されました。AviUtl2を再起動すると反映されます。",
            )
            .unwrap(),
        )
    {
        tracing::warn!(?err, "failed to show update notification");
    }

    Ok(())
}

fn fetch_remote_index() -> anyhow::Result<RemoteFilesIndex> {
    let bytes = http::fetch_url_bytes(http::FILES_INDEX_URL)?;
    let index = serde_json::from_slice::<RemoteFilesIndex>(&bytes)?;
    Ok(index)
}

fn hash_file_hex(path: &Path) -> anyhow::Result<String> {
    let data = fs::read(path)?;
    Ok(hash_bytes_hex(&data))
}

fn hash_bytes_hex(data: &[u8]) -> String {
    format!("{:016x}", xxhash_rust::xxh3::xxh3_64(data))
}

fn write_atomic(path: &Path, data: &[u8]) -> anyhow::Result<()> {
    let tmp_path = temp_path_for(path);
    fs::write(&tmp_path, data)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(tmp_path, path)?;
    Ok(())
}

fn temp_path_for(path: &Path) -> PathBuf {
    let file_name = path.file_name().and_then(|x| x.to_str()).unwrap_or("tmp");
    path.with_file_name(format!("{file_name}.tmp"))
}

fn copy_language_overlay_files_if_needed(language_dir: &Path) -> anyhow::Result<bool> {
    let mut copied = false;
    for entry in fs::read_dir(language_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        tracing::debug!(file_name = ?entry.file_name(), "checking for language overlay file");

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let Some((language_code, suffix)) = parse_language_overlay(&file_name) else {
            tracing::debug!(?file_name, "not a recognized language overlay file, skipping");
            continue;
        };

        let source_path = entry.path();
        let dest_path =
            language_dir.join(format!("community_{language_code}.{suffix}.copied.aul2"));

        let source_hash = hash_file_hex(&source_path)?;
        let dest_hash = hash_file_hex(&dest_path).ok();
        if dest_hash.as_deref() == Some(source_hash.as_str()) {
            tracing::debug!(
                ?file_name,
                "compatible language overlay file already exists, skipping copy"
            );
            continue;
        }

        tracing::info!(?file_name, "copying language overlay file");
        let source_bytes = fs::read(&source_path)?;
        write_atomic(&dest_path, &source_bytes)?;
        copied = true;
    }

    Ok(copied)
}

fn parse_language_overlay(file_name: &str) -> Option<(&'static str, &str)> {
    if !file_name.ends_with(".aul2") {
        return None;
    }

    let end = file_name.len() - ".aul2".len();
    let dot_index = file_name.find('.')?;
    let language_name = &file_name[..dot_index];
    let language_code = map_language_name_to_code(language_name)?;
    let start = dot_index + ".".len();

    if start >= end {
        return None;
    }

    Some((language_code, &file_name[start..end]))
}

fn map_language_name_to_code(language_name: &str) -> Option<&'static str> {
    match language_name {
        "English" => Some("en"),
        "Japanese" => Some("ja"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{map_language_name_to_code, parse_language_overlay};

    #[test]
    fn parses_known_language_overlay() {
        assert_eq!(
            parse_language_overlay("English.script.aul2"),
            Some(("en", "script"))
        );
        assert_eq!(
            parse_language_overlay("German.plugin.patch.aul2"),
            Some(("de", "plugin.patch"))
        );
    }

    #[test]
    fn rejects_unknown_or_invalid_overlay() {
        assert_eq!(parse_language_overlay("French.script.aul2"), None);
        assert_eq!(parse_language_overlay("English.aul2"), None);
        assert_eq!(parse_language_overlay("English.script.txt"), None);
    }

    #[test]
    fn maps_language_names_to_codes() {
        assert_eq!(map_language_name_to_code("English"), Some("en"));
        assert_eq!(map_language_name_to_code("Japanese"), Some("ja"));
        assert_eq!(map_language_name_to_code("German"), Some("de"));
        assert_eq!(map_language_name_to_code("Spanish"), Some("es-ES"));
    }
}
