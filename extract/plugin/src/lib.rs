use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use aviutl2::{
    AviUtl2Info,
    generic::{EditHandle, GenericPlugin, GenericPluginTable, HostAppHandle, ProjectFile},
};

#[aviutl2::plugin(GenericPlugin)]
struct ExtractPlugin {
    data_root: PathBuf,
    edit_handle: Option<Arc<EditHandle>>,
}

impl ExtractPlugin {
    fn mode_path(&self) -> PathBuf {
        self.data_root.join("mode.txt")
    }

    fn effects_path(&self) -> PathBuf {
        self.data_root.join("effects")
    }

    fn write_shutdown(&self) {
        let _ = fs::write(self.mode_path(), "shutdown");
    }

    fn restart_host_app(&self) {
        if let Some(edit_handle) = &self.edit_handle {
            edit_handle.restart_host_app();
        }
    }

    fn defer_collect_effects(&self) {
        let Some(edit_handle) = self.edit_handle.clone() else {
            return;
        };
        let effects_path = self.effects_path();
        let mode_path = self.mode_path();

        thread::spawn(move || {
            collect_effects(edit_handle.clone(), &effects_path);
            let _ = fs::write(mode_path, "shutdown");
            edit_handle.restart_host_app();
        });
    }
}

impl GenericPlugin for ExtractPlugin {
    fn new(_info: AviUtl2Info) -> aviutl2::AnyResult<Self> {
        let dylib_path = process_path::get_dylib_path()
            .ok_or_else(|| aviutl2::anyhow::anyhow!("failed to get plugin path"))?;
        let data_root = dylib_path
            .parent()
            .and_then(Path::parent)
            .ok_or_else(|| aviutl2::anyhow::anyhow!("failed to get plugin data root"))?
            .join("extract");

        if !data_root.join("mode.txt").exists() {
            return Err(aviutl2::anyhow::anyhow!("mode.txt does not exist"));
        }

        Ok(Self {
            data_root,
            edit_handle: None,
        })
    }

    fn plugin_info(&self) -> GenericPluginTable {
        GenericPluginTable {
            name: "ExtractPlugin".to_owned(),
            information: "Extract Plugin".to_owned(),
        }
    }

    fn register(&mut self, registry: &mut HostAppHandle) {
        self.edit_handle = Some(Arc::new(registry.create_edit_handle()));
    }

    fn on_project_load(&mut self, _project: &mut ProjectFile) {
        let mode = fs::read_to_string(self.mode_path()).unwrap_or_default();
        match mode.trim_end() {
            "extract" => self.defer_collect_effects(),
            "reboot" => {
                self.write_shutdown();
                self.restart_host_app();
            }
            "shutdown" => {
                let _ = fs::remove_file(self.mode_path());
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

#[derive(serde::Serialize)]
struct EffectData {
    name: String,
    parameters: Vec<String>,
}

fn collect_effects(edit_handle: Arc<EditHandle>, effects_path: &Path) {
    let _ = fs::create_dir_all(effects_path);
    for (index, effect) in edit_handle.get_effects().into_iter().enumerate() {
        let output_path = effects_path.join(format!("{}.json", index + 1));
        let mut names = vec![];
        for param in edit_handle.get_effect_items(&effect.name).unwrap() {
            names.push(param.name.clone());
        }
        let data = EffectData {
            name: effect.name.clone(),
            parameters: names,
        };
        if let Ok(json) = serde_json::to_string_pretty(&data) {
            let _ = fs::write(output_path, json);
        }
    }
}

aviutl2::register_generic_plugin!(ExtractPlugin);
