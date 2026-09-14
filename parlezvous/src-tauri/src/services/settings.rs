use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub target_language: String,
    pub tts_server_url: String,
    pub asr_server_url: String,
    pub ai_provider: String,
    pub ai_base_url: String,
    pub ai_api_key: String,
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
    pub tutor_tone: String,
}

pub fn get_settings(db: Arc<Mutex<Connection>>) -> Result<AppSettings, String> {
    let conn = db.lock().map_err(|_| "DB lock failed")?;
    let sql = "SELECT target_language, tts_server_url, asr_server_url, ai_provider, ai_base_url, ai_api_key, embedding_model, active_model, huggingface_token, litert_accelerator, litert_max_tokens, target_programming_language, coding_theme_category, active_vrm, supertonic_voice_style, tts_provider, tutor_tone FROM settings WHERE id = 1";
    conn.query_row(sql, [], |row| {
        Ok(AppSettings {
            target_language: row.get(0)?,
            tts_server_url: row.get(1)?,
            asr_server_url: row.get(2)?,
            ai_provider: row.get(3).unwrap_or_else(|_| "managed".into()),
            ai_base_url: row.get(4).unwrap_or_else(|_| "http://127.0.0.1:11435/v1".into()),
            ai_api_key: row.get(5).unwrap_or_default(),
            embedding_model: row.get(6)?,
            active_model: row.get(7)?,
            huggingface_token: row.get(8)?,
            litert_accelerator: row.get(9).unwrap_or_else(|_| "Auto".into()),
            litert_max_tokens: row.get(10).unwrap_or(1024),
            target_programming_language: row.get(11).unwrap_or_else(|_| "python".into()),
            coding_theme_category: row.get(12).unwrap_or_else(|_| "All".into()),
            active_vrm: row.get(13).unwrap_or_else(|_| "avatar.vrm".into()),
            supertonic_voice_style: row.get(14).unwrap_or_else(|_| "voice_styles/F1.json".into()),
            tts_provider: row.get(15).unwrap_or_else(|_| "auto".into()),
            tutor_tone: row.get(16).unwrap_or_else(|_| "balanced".into()),
        })
    }).map_err(|error| error.to_string())
}

pub fn update_settings(db: Arc<Mutex<Connection>>, settings: AppSettings) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "DB lock failed")?;
    conn.execute(
        "UPDATE settings SET target_language=?1, tts_server_url=?2, asr_server_url=?3, ai_provider=?4, ai_base_url=?5, ai_api_key=?6, embedding_model=?7, active_model=?8, huggingface_token=?9, litert_accelerator=?10, litert_max_tokens=?11, target_programming_language=?12, coding_theme_category=?13, active_vrm=?14, supertonic_voice_style=?15, tts_provider=?16, tutor_tone=?17 WHERE id=1",
        rusqlite::params![
            settings.target_language, settings.tts_server_url, settings.asr_server_url,
            settings.ai_provider, settings.ai_base_url, settings.ai_api_key, settings.embedding_model,
            settings.active_model, settings.huggingface_token, settings.litert_accelerator,
            settings.litert_max_tokens, settings.target_programming_language, settings.coding_theme_category,
            settings.active_vrm, settings.supertonic_voice_style, settings.tts_provider, settings.tutor_tone,
        ],
    ).map_err(|error| error.to_string())?;
    Ok(())
}
