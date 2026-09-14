use crate::AppState;
use tauri::{Manager, State};

#[tauri::command]
pub(crate) async fn check_ai_health(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.router.remote().check_health().await)
}

#[tauri::command]
pub(crate) async fn list_ai_models(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    state.router.remote().list_models().await
}


#[tauri::command]
pub(crate) async fn get_managed_ai_status(
    app: tauri::AppHandle,
    runtime: State<'_, crate::services::managed_ai::ManagedAiRuntime>,
) -> Result<crate::services::managed_ai::ManagedAiStatus, String> {
    crate::services::managed_ai::status(&app, &runtime).await
}

#[tauri::command]
pub(crate) async fn install_managed_ai(
    app: tauri::AppHandle,
    runtime: State<'_, crate::services::managed_ai::ManagedAiRuntime>,
) -> Result<(), String> {
    crate::services::managed_ai::install(&app, &runtime).await
}

#[tauri::command]
pub(crate) fn start_managed_ai(
    app: tauri::AppHandle,
    runtime: State<'_, crate::services::managed_ai::ManagedAiRuntime>,
) -> Result<(), String> {
    crate::services::managed_ai::start(&app, &runtime)
}

#[tauri::command]
pub(crate) async fn install_managed_vision(
    app: tauri::AppHandle,
    runtime: State<'_, crate::services::managed_ai::ManagedAiRuntime>,
) -> Result<(), String> {
    crate::services::managed_ai::install_vision(&app, &runtime).await
}

#[tauri::command]
pub(crate) async fn get_settings(
    state: State<'_, AppState>,
) -> Result<crate::services::settings::AppSettings, String> {
    crate::services::settings::get_settings(state.db.clone())
}

#[tauri::command]
pub(crate) async fn update_settings(
    state: State<'_, AppState>,
    settings: crate::services::settings::AppSettings,
) -> Result<(), String> {
    crate::services::settings::update_settings(state.db.clone(), settings)
}

#[tauri::command]
pub(crate) async fn check_tokenizer_exists(app_handle: tauri::AppHandle) -> Result<bool, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let tokenizer_path = app_dir.join("tokenizer.json");
    let config_path = app_dir.join("tokenizer_config.json");

    Ok(tokenizer_path.exists() && config_path.exists())
}

#[tauri::command]
pub(crate) async fn download_tokenizer(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    let tokenizer_path = app_dir.join("tokenizer.json");
    let config_path = app_dir.join("tokenizer_config.json");

    let client = reqwest::Client::new();

    let res = client.get("https://huggingface.co/google/gemma-4-E2B/resolve/main/tokenizer.json")
        .send().await.map_err(|e| format!("Failed to fetch tokenizer.json: {}", e))?;
    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(&tokenizer_path, bytes).map_err(|e| e.to_string())?;

    let res = client.get("https://huggingface.co/google/gemma-4-E2B/resolve/main/tokenizer_config.json")
        .send().await.map_err(|e| format!("Failed to fetch tokenizer_config.json: {}", e))?;
    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(&config_path, bytes).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub(crate) async fn delete_tokenizer(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let tokenizer_path = app_dir.join("tokenizer.json");
    let config_path = app_dir.join("tokenizer_config.json");

    let _ = std::fs::remove_file(tokenizer_path);
    let _ = std::fs::remove_file(config_path);

    Ok(())
}
