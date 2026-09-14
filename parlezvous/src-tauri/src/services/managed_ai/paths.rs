use std::path::PathBuf;
use tauri::Manager;

#[derive(Clone)]
pub struct ManagedAiPaths {
    pub root: PathBuf,
    pub runtime: PathBuf,
    pub models: PathBuf,
    pub server: PathBuf,
    pub chat: PathBuf,
    pub embedding: PathBuf,
    pub vision: PathBuf,
    pub vision_mmproj: PathBuf,
    pub presets: PathBuf,
}

impl ManagedAiPaths {
    pub fn resolve(app: &tauri::AppHandle) -> Result<Self, String> {
        let root = app.path().app_data_dir().map_err(|error| error.to_string())?.join("managed-ai");
        let runtime = root.join("runtime");
        let models = root.join("models");
        Ok(Self {
            server: runtime.join(super::manifest::server_binary_name()),
            chat: models.join("qwen3.5-4b-q4.gguf"),
            embedding: models.join("qwen3-embedding-0.6b-q8.gguf"),
            vision: models.join("minicpm-v-4.6-q4.gguf"),
            vision_mmproj: models.join("minicpm-v-4.6-mmproj.gguf"),
            presets: root.join("models.ini"),
            root, runtime, models,
        })
    }

    pub fn ensure_dirs(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.runtime).map_err(|error| error.to_string())?;
        std::fs::create_dir_all(&self.models).map_err(|error| error.to_string())?;
        Ok(())
    }
}
