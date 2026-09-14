use crate::{ai::{ChatMessage, ChatResponse}, AppState};
use tauri::State;

#[tauri::command]
pub(crate) async fn chat_with_avatar(
    state: State<'_, AppState>,
    history: Vec<ChatMessage>,
    model: String,
    language: String,
    active_textbook: Option<String>,
    active_page: Option<i32>,
    active_theme: Option<String>,
    active_subtheme: Option<String>,
    audio_base64: Option<String>,
    image_uri: Option<String>,
) -> Result<ChatResponse, String> {
    println!(
        "[IPC] chat_with_avatar called for model {} in {}",
        model, language
    );

    let context_str = if let Some(book) = active_textbook {
        let last_user_msg = history
            .iter()
            .rev()
            .find(|m| m.role == "user")
            .map(|m| m.content.clone())
            .unwrap_or_default();
        if !last_user_msg.is_empty() {
            let settings = crate::services::settings::get_settings(state.db.clone())?;
            let chunks = crate::services::rag::query_context(
                state.db.clone(),
                state.router.embeddings(),
                book,
                last_user_msg,
                settings.embedding_model,
                active_page,
                active_theme.clone(),
            )
            .await?;
            chunks.join("\n\n")
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let skill_level = state
        .skill_level
        .read()
        .map_err(|_| "Failed to lock skill_level")?
        .clone();
    let ai = state.router.provider_for(&model)?;
    ai.generate_chat_response(
            history,
            model,
            language,
            skill_level,
            context_str,
            active_theme,
            active_subtheme,
            audio_base64,
            image_uri,
        )
        .await
}

#[tauri::command]
pub(crate) async fn get_chat_history(
    state: State<'_, AppState>,
) -> Result<Vec<crate::services::chat_history::ChatHistoryMessage>, String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        crate::services::chat_history::list(&conn)
    }).await.map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn save_chat_message(
    state: State<'_, AppState>,
    role: String,
    content: String,
    correction: Option<String>,
    audio_base64: Option<String>,
) -> Result<i64, String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        crate::services::chat_history::insert(
            &conn, &role, &content, correction.as_deref(), audio_base64.as_deref()
        )
    }).await.map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn update_chat_message_correction(
    state: State<'_, AppState>,
    id: i64,
    correction: String,
) -> Result<(), String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        crate::services::chat_history::update_user_correction(&conn, id, &correction)
    }).await.map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn clear_chat_history(state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        crate::services::chat_history::clear(&conn)
    }).await.map_err(|error| error.to_string())?
}
