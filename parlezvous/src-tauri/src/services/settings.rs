use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSettings {
    pub target_language: String,
    pub tts_server_url: String,
    pub asr_server_url: String,
    pub ollama_server_url: String,
    pub embedding_model: String,
    pub active_model: String,
    pub huggingface_token: Option<String>,
    pub litert_accelerator: String,
    pub litert_max_tokens: u32,
    pub target_programming_language: String,
    pub coding_theme_category: String,
    pub active_vrm: String,
    pub supertonic_voice_style: String,
    pub tts_provider: String,
}

pub fn get_settings(db: Arc<Mutex<Connection>>) -> Result<AppSettings, String> {
    let conn = db.lock().map_err(|_| "DB lock failed")?;

    let mut stmt = conn.prepare("SELECT target_language, tts_server_url, asr_server_url, ollama_server_url, embedding_model, active_model, huggingface_token, litert_accelerator, litert_max_tokens, target_programming_language, coding_theme_category, active_vrm, supertonic_voice_style, tts_provider FROM settings WHERE id = 1").map_err(|e| e.to_string())?;
    let settings = stmt
        .query_row([], |row| {
            Ok(AppSettings {
                target_language: row.get(0)?,
                tts_server_url: row.get(1)?,
                asr_server_url: row.get(2)?,
                ollama_server_url: row.get(3)?,
                embedding_model: row.get(4)?,
                active_model: row.get(5)?,
                huggingface_token: row.get(6)?,
                litert_accelerator: row.get(7).unwrap_or_else(|_| "Auto".to_string()),
                litert_max_tokens: row.get(8).unwrap_or(5000),
                target_programming_language: row.get(9).unwrap_or_else(|_| "python".to_string()),
                coding_theme_category: row.get(10).unwrap_or_else(|_| "All".to_string()),
                active_vrm: row.get(11).unwrap_or_else(|_| "avatar.vrm".to_string()),
                supertonic_voice_style: row.get(12).unwrap_or_else(|_| "voice_styles/F1.json".to_string()),
                tts_provider: row.get(13).unwrap_or_else(|_| "auto".to_string()),
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(settings)
}

pub fn update_settings(db: Arc<Mutex<Connection>>, settings: AppSettings) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "DB lock failed")?;

    conn.execute(
        "UPDATE settings SET target_language = ?1, tts_server_url = ?2, asr_server_url = ?3, ollama_server_url = ?4, embedding_model = ?5, active_model = ?6, huggingface_token = ?7, litert_accelerator = ?8, litert_max_tokens = ?9, target_programming_language = ?10, coding_theme_category = ?11, active_vrm = ?12, supertonic_voice_style = ?13, tts_provider = ?14 WHERE id = 1",
        (&settings.target_language, &settings.tts_server_url, &settings.asr_server_url, &settings.ollama_server_url, &settings.embedding_model, &settings.active_model, &settings.huggingface_token, &settings.litert_accelerator, &settings.litert_max_tokens, &settings.target_programming_language, &settings.coding_theme_category, &settings.active_vrm, &settings.supertonic_voice_style, &settings.tts_provider),
    ).map_err(|e| e.to_string())?;

    Ok(())
}
