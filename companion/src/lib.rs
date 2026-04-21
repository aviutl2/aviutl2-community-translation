use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Once;

use serde::Deserialize;
use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};

const FILES_INDEX_URL: &str = "https://raw.githubusercontent.com/aviutl2/aviutl2-community-translation/main/locales/files.json";
const LOCALES_BASE_URL: &str =
    "https://raw.githubusercontent.com/aviutl2/aviutl2-community-translation/main/locales/";

static UPDATE_ONCE: Once = Once::new();

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Deserialize)]
struct RemoteFilesIndex {
    version: u32,
    files: BTreeMap<String, String>,
}

#[aviutl2::plugin(GenericPlugin)]
struct TranslationCompanion {}

impl aviutl2::generic::GenericPlugin for TranslationCompanion {
    fn new(_info: aviutl2::common::AviUtl2Info) -> aviutl2::common::AnyResult<Self> {
        Ok(Self {})
    }

    fn plugin_info(&self) -> aviutl2::generic::GenericPluginTable {
        aviutl2::generic::GenericPluginTable {
            name: "aviutl2-community-translation companion".to_string(),
            information: "https://github.com/aviutl2/aviutl2-community-translation".to_string(),
        }
    }

    fn register(&mut self, _registry: &mut aviutl2::generic::HostAppHandle) {}

    fn on_project_load(&mut self, _project: &mut aviutl2::generic::ProjectFile) {
        UPDATE_ONCE.call_once(|| {
            std::thread::spawn(|| {
                if let Err(err) = run_update_task() {
                    let _ =
                        aviutl2::logger::write_warn_log(&format!("companion update failed: {err}"));
                }
            });
        });
    }
}

aviutl2::register_generic_plugin!(TranslationCompanion);

fn run_update_task() -> AppResult<()> {
    let language_dir = aviutl2::config::app_data_path().join("Language");
    fs::create_dir_all(&language_dir)?;

    let index = fetch_remote_index()?;
    if index.version != 1 {
        let _ = aviutl2::logger::write_warn_log(&format!(
            "unsupported files index version: {}",
            index.version
        ));
        return Ok(());
    }

    let mut changed = false;
    for (file_name, remote_hash) in &index.files {
        if !file_name.ends_with(".aul2") {
            continue;
        }

        let local_path = language_dir.join(file_name);
        let local_hash = hash_file_hex(&local_path).ok();
        if local_hash.as_deref() == Some(remote_hash.as_str()) {
            continue;
        }

        let url = format!("{LOCALES_BASE_URL}{file_name}");
        let file_bytes = fetch_url_bytes(&url)?;
        let fetched_hash = hash_bytes_hex(&file_bytes);
        if fetched_hash != *remote_hash {
            let _ = aviutl2::logger::write_warn_log(&format!(
                "hash mismatch for {file_name}: expected {remote_hash}, got {fetched_hash}"
            ));
            continue;
        }

        write_atomic(&local_path, &file_bytes)?;
        changed = true;
    }

    if copy_english_files_if_needed(&language_dir)? {
        changed = true;
    }

    if changed {
        if let Err(err) = show_update_toast(
            "AviUtl2 Community Translation",
            "翻訳ファイルが更新されました。",
        ) {
            let _ = aviutl2::logger::write_warn_log(&format!("failed to show toast: {err}"));
        }
    }

    Ok(())
}

fn fetch_remote_index() -> AppResult<RemoteFilesIndex> {
    let bytes = fetch_url_bytes(FILES_INDEX_URL)?;
    let index = serde_json::from_slice::<RemoteFilesIndex>(&bytes)?;
    Ok(index)
}

fn fetch_url_bytes(url: &str) -> AppResult<Vec<u8>> {
    let response = ureq::get(url).call()?;
    let mut reader = response.into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn hash_file_hex(path: &Path) -> AppResult<String> {
    let data = fs::read(path)?;
    Ok(hash_bytes_hex(&data))
}

fn hash_bytes_hex(data: &[u8]) -> String {
    format!("{:016x}", xxhash_rust::xxh3::xxh3_64(data))
}

fn write_atomic(path: &Path, data: &[u8]) -> AppResult<()> {
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

fn copy_english_files_if_needed(language_dir: &Path) -> AppResult<bool> {
    let mut copied = false;
    for entry in fs::read_dir(language_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let Some(suffix) = parse_english_suffix(&file_name) else {
            continue;
        };

        let source_path = entry.path();
        let dest_path = language_dir.join(format!("community_en.{suffix}.copied.aul2"));

        let source_hash = hash_file_hex(&source_path)?;
        let dest_hash = hash_file_hex(&dest_path).ok();
        if dest_hash.as_deref() == Some(source_hash.as_str()) {
            continue;
        }

        let source_bytes = fs::read(&source_path)?;
        write_atomic(&dest_path, &source_bytes)?;
        copied = true;
    }

    Ok(copied)
}

fn parse_english_suffix(file_name: &str) -> Option<&str> {
    if !file_name.starts_with("English.") || !file_name.ends_with(".aul2") {
        return None;
    }

    let start = "English.".len();
    let end = file_name.len() - ".aul2".len();
    if start >= end {
        return None;
    }

    Some(&file_name[start..end])
}

fn show_update_toast(title: &str, body: &str) -> AppResult<()> {
    let _apartment = windows::core::initialize_mta()?;
    let toast_xml = format!(
        "<toast><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
        escape_xml(title),
        escape_xml(body)
    );

    let xml = XmlDocument::new()?;
    xml.LoadXml(&toast_xml.into())?;

    let toast = ToastNotification::CreateToastNotification(&xml)?;
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(
        &"aviutl2-community-translation-companion".into(),
    )?;
    notifier.Show(&toast)?;
    Ok(())
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
