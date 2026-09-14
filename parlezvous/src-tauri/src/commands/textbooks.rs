use crate::AppState;
use tauri::{Manager, State};

#[tauri::command]
pub(crate) async fn upload_and_ingest_textbook(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    file_path: String,
    model: String,
) -> Result<(), String> {
    // Copy the PDF into app_data_dir/textbooks/ so the webview can access it
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let textbooks_dir = app_dir.join("textbooks");
    std::fs::create_dir_all(&textbooks_dir).map_err(|e| e.to_string())?;

    let source = std::path::Path::new(&file_path);
    let file_name = source
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dest = textbooks_dir.join(&file_name);

    if !dest.exists() {
        std::fs::copy(&source, &dest).map_err(|e| format!("Failed to copy PDF: {}", e))?;
        println!("[RAG] Copied PDF to: {:?}", dest);
    }

    crate::services::rag::ingest_pdf(state.db.clone(), state.router.embeddings(), file_path, model).await
}

#[tauri::command]
pub(crate) async fn get_textbook_dir(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let textbooks_dir = app_dir.join("textbooks");
    std::fs::create_dir_all(&textbooks_dir).map_err(|e| e.to_string())?;
    Ok(textbooks_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub(crate) async fn list_textbooks(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    crate::services::rag::list_textbooks(state.db.clone()).await
}
