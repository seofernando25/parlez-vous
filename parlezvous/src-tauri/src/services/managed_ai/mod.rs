mod download;
pub mod manifest;
mod paths;
mod runtime;

pub use runtime::ManagedAiRuntime;
use paths::ManagedAiPaths;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedAiProgress { pub stage: String, pub downloaded: u64, pub total: u64 }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedAiStatus {
    pub supported: bool,
    pub runtime_installed: bool,
    pub chat_model_installed: bool,
    pub embedding_model_installed: bool,
    pub vision_model_installed: bool,
    pub running: bool,
    pub ready: bool,
}

pub async fn status(app: &tauri::AppHandle, runtime: &ManagedAiRuntime) -> Result<ManagedAiStatus, String> {
    let paths = ManagedAiPaths::resolve(app)?;
    let supported = manifest::runtime_asset_fragment().is_some();
    let running = runtime.is_running();
    let ready = if running {
        model_catalog_ready().await
    } else { false };
    Ok(ManagedAiStatus {
        supported,
        runtime_installed: paths.server.exists(),
        chat_model_installed: paths.chat.exists(),
        embedding_model_installed: paths.embedding.exists(),
        vision_model_installed: paths.vision.exists() && paths.vision_mmproj.exists(),
        running,
        ready,
    })
}

pub async fn install(app: &tauri::AppHandle, runtime: &ManagedAiRuntime) -> Result<(), String> {
    let paths = ManagedAiPaths::resolve(app)?;
    runtime.stop();
    download::install(app, &paths).await?;
    runtime.start(&paths)
}

pub async fn install_vision(app: &tauri::AppHandle, runtime: &ManagedAiRuntime) -> Result<(), String> {
    let paths = ManagedAiPaths::resolve(app)?;
    runtime.stop();
    download::install_vision(app, &paths).await?;
    runtime.start(&paths)
}

pub fn start(app: &tauri::AppHandle, runtime: &ManagedAiRuntime) -> Result<(), String> {
    let paths = ManagedAiPaths::resolve(app)?;
    if paths.server.exists() && paths.chat.exists() && paths.embedding.exists() {
        download::write_presets(&paths)?;
    }
    runtime.start(&paths)
}

async fn model_catalog_ready() -> bool {
    let url = format!("{}/models", manifest::MANAGED_BASE_URL);
    let Ok(response) = reqwest::Client::new().get(url).send().await else { return false; };
    let Ok(value) = response.json::<serde_json::Value>().await else { return false; };
    let ids: Vec<&str> = value.get("data").and_then(|v| v.as_array()).into_iter().flatten()
        .filter_map(|item| item.get("id").and_then(|id| id.as_str())).collect();
    ids.contains(&manifest::CHAT_ALIAS) && ids.contains(&manifest::EMBEDDING_ALIAS)
}
