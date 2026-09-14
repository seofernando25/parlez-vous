use crate::{ai::{GradingVariables, JournalResponse, JournalVariables}, db::VocabId, services::journal::{process_journal_generation, process_journal_grading, DbVocabItem}, AppState};
use tauri::{Emitter, State};

#[tauri::command]
pub(crate) async fn generate_journal(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    mood: String,
    weather: String,
    activity: String,
    model: String,
    language: String,
    active_theme: Option<String>,
) -> Result<JournalResponse, String> {
    println!(
        "[IPC] generate_journal called with mood: {}, weather: {}, activity: {}, language: {}",
        mood, weather, activity, language
    );
    let skill_level = state
        .skill_level
        .read()
        .map_err(|_| "Failed to lock skill_level")?
        .clone();
    let ai = state.router.provider_for(&model)?;
    let variables = JournalVariables {
        mood,
        weather,
        activity,
        model,
        language,
        skill_level,
        active_theme,
    };

    let (response, db_vocab_chips) =
        process_journal_generation(state.db.clone(), ai, variables).await?;

    app_handle
        .emit("vocabulary_extracted", &db_vocab_chips)
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
pub(crate) async fn grade_journal(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    entry: String,
    model: String,
    language: String,
) -> Result<JournalResponse, String> {
    println!("[IPC] grade_journal called for language: {}", language);
    let skill_level = state
        .skill_level
        .read()
        .map_err(|_| "Failed to lock skill_level")?
        .clone();
    let ai = state.router.provider_for(&model)?;
    let variables = GradingVariables {
        entry,
        model,
        language,
        skill_level,
    };

    let (response, db_vocab_chips) =
        process_journal_grading(state.db.clone(), ai, variables).await?;

    app_handle
        .emit("vocabulary_extracted", &db_vocab_chips)
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
pub(crate) async fn get_journal_entries(
    state: State<'_, AppState>,
) -> Result<Vec<crate::services::calendar::JournalEntryDTO>, String> {
    crate::services::calendar::get_journal_entries(state.db.clone()).await
}

#[tauri::command]
pub(crate) async fn get_all_vocabulary(state: State<'_, AppState>) -> Result<Vec<DbVocabItem>, String> {
    println!("[IPC] get_all_vocabulary called");
    crate::services::vocab::get_vocabulary(state.db.clone()).await
}

#[tauri::command]
pub(crate) async fn add_to_srs(state: State<'_, AppState>, vocab_id: VocabId) -> Result<(), String> {
    println!("[IPC] add_to_srs called with vocab_id: {:?}", vocab_id);
    let db = state.db.clone();

    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        conn.execute(
            "INSERT OR IGNORE INTO srs_state (vocab_id, review_level, next_review_date, ease_factor, interval_days) VALUES (?1, 1, date('now', '+1 day'), 2.5, 1)",
            [vocab_id]
        ).map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    Ok(())
}
