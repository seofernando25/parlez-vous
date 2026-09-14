use crate::AppState;
use tauri::State;

#[derive(serde::Serialize)]
pub(crate) struct ConjugationResponse {
    exercise: crate::ai::ConjugationExercise,
    history_id: i64,
}

#[tauri::command]
pub(crate) async fn generate_conjugation_exercise(
    state: State<'_, AppState>,
    language: String,
    model: String,
) -> Result<ConjugationResponse, String> {
    println!("[IPC] generate_conjugation_exercise called");

    let db = state.db.clone();
    let ai = state.router.provider_for(&model)?;

    let handle = tokio::spawn(async move {
        crate::services::conjugator::process_conjugation_generation(
            db,
            ai,
            language,
            model,
        )
        .await
    });

    {
        let mut task_guard = state.conjugation_task.lock().await;
        if let Some(old_abort) = task_guard.replace(handle.abort_handle()) {
            old_abort.abort();
        }
    }

    match handle.await {
        Ok(Ok((exercise, history_id))) => Ok(ConjugationResponse { exercise, history_id }),
        Ok(Err(e)) => Err(e),
        Err(e) if e.is_cancelled() => Err("Cancelled".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub(crate) async fn cancel_conjugation_generation(state: State<'_, AppState>) -> Result<(), String> {
    println!("[IPC] cancel_conjugation_generation called");
    let mut task_guard = state.conjugation_task.lock().await;
    if let Some(abort_handle) = task_guard.take() {
        abort_handle.abort();
    }
    Ok(())
}

#[tauri::command]
pub(crate) async fn record_conjugation_result(
    state: State<'_, AppState>,
    history_id: i64,
    correct: bool,
) -> Result<(), String> {
    println!(
        "[IPC] record_conjugation_result called (id={}, correct={})",
        history_id, correct
    );
    crate::services::conjugator::record_conjugation_result(state.db.clone(), history_id, correct)
        .await
}

#[tauri::command]
pub(crate) async fn get_all_tense_stats(
    state: State<'_, AppState>,
    language: String,
) -> Result<Vec<crate::ai::TenseStat>, String> {
    crate::services::conjugator::get_all_tense_stats(state.db.clone(), language)
}
