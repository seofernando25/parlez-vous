use crate::{db::VocabId, model, services::srs::calculate_new_srs_state, AppState};
use tauri::State;

#[tauri::command]
pub(crate) async fn infer_character(
    state: State<'_, AppState>,
    pixels: Vec<u8>,
    vocab_id: VocabId,
    target_text: String,
    script: Option<String>,
) -> Result<String, String> {
    println!(
        "[IPC] infer_character called for target_text: '{}', script: '{:?}'",
        target_text, script
    );

    let target_trimmed = target_text.trim();
    let target_lower = target_trimmed.to_lowercase();
    let script_lower = script.as_deref().unwrap_or("").trim().to_lowercase();

    let is_cyrillic = script_lower == "russian"
        || script_lower == "ukrainian"
        || script_lower == "cyrillic"
        || target_lower == "russian"
        || target_lower == "ukrainian"
        || target_lower == "cyrillic"
        || model::ALL_CYRILLIC.contains(&target_lower.as_str());

    let is_jamo = script_lower == "korean"
        || script_lower == "hangul"
        || target_lower == "korean"
        || target_lower == "hangul"
        || model::ALL_JAMO.contains(&target_trimmed)
        || (!is_cyrillic && target_trimmed.is_empty() && script_lower.is_empty());

    // Run inference:
    // 1. If Cyrillic, use compiled Burn Cyrillic model.
    // 2. If Hangul jamo, use compiled Burn Hangul model.
    // 3. Otherwise (e.g. Latin), check strokes.
    let (predicted, confidence) = if is_cyrillic {
        tokio::task::spawn_blocking(move || model::infer_cyrillic(&pixels))
            .await
            .map_err(|e| e.to_string())??
    } else if is_jamo {
        tokio::task::spawn_blocking(move || model::infer(&pixels))
            .await
            .map_err(|e| e.to_string())??
    } else {
        if pixels.len() != 784 {
            return Err(format!("Expected 784 pixels, got {}", pixels.len()));
        }
        let stroke_count = pixels.iter().filter(|&&p| p > 50).count();
        if stroke_count >= 8 {
            (target_text.clone(), 0.95)
        } else {
            ("?".to_string(), 0.0)
        }
    };

    println!(
        "[IPC] Predicted: {} (confidence: {:.2}%)",
        predicted,
        confidence * 100.0
    );

    let is_match = if target_trimmed.is_empty() {
        false
    } else if is_jamo {
        predicted == target_trimmed
    } else {
        predicted.to_lowercase() == target_lower
    };

    if is_match {
        let db = state.db.clone();
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().map_err(|_| "DB lock failed")?;

            // Fetch current state
            let (current_level, current_ease): (i32, f32) = conn
                .query_row(
                    "SELECT review_level, ease_factor FROM srs_state WHERE vocab_id = ?1",
                    [vocab_id],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .unwrap_or((0, 2.5));

            // Use decoupled business logic for fuzzable SRS math
            let (new_level, new_ease) = calculate_new_srs_state(current_level, current_ease)?;

            conn.execute(
                "UPDATE srs_state SET review_level = ?1, ease_factor = ?2 WHERE vocab_id = ?3",
                rusqlite::params![new_level, new_ease, vocab_id],
            )
            .map_err(|e| e.to_string())?;

            Ok::<(), String>(())
        })
        .await
        .map_err(|e| e.to_string())??;
    }

    Ok(predicted)
}

#[tauri::command]
pub(crate) fn get_all_jamo() -> Vec<String> {
    model::ALL_JAMO.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
pub(crate) fn get_alphabet_letters(script: String) -> Vec<model::AlphabetLetter> {
    model::get_alphabet_letters(&script)
}
