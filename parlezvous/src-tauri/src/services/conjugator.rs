use crate::ai::{ConjugationExercise, LlmProvider, TenseStat};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

/// Fetch the most recent conjugation exercises for the given language from the DB.
/// Returns formatted strings like "avoir (Present, je)" to feed into the LLM prompt.
fn get_recent_conjugations(
    conn: &Connection,
    language: &str,
    limit: usize,
) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT verb FROM conjugation_history \
         WHERE language_code = ?1 \
         ORDER BY created_at DESC LIMIT ?2",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![language, limit as i64], |row| {
            let verb: String = row.get(0)?;
            Ok(verb)
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for row in rows {
        let v = row.map_err(|e| e.to_string())?;
        if !results.contains(&v) {
            results.push(v);
        }
    }
    Ok(results)
}

/// Compute per-tense accuracy stats from the last `limit` answered exercises for the language.
fn get_tense_stats(
    conn: &Connection,
    language: &str,
    limit: usize,
) -> Result<Vec<TenseStat>, String> {
    // Sub-query limits to the last N answered rows, then groups by tense.
    let mut stmt = conn.prepare(
        "SELECT tense, COUNT(*) as total, SUM(CASE WHEN answered_correct = 1 THEN 1 ELSE 0 END) as correct \
         FROM ( \
             SELECT tense, answered_correct FROM conjugation_history \
             WHERE language_code = ?1 AND answered_correct IS NOT NULL \
             ORDER BY created_at DESC LIMIT ?2 \
         ) GROUP BY tense ORDER BY total DESC"
    ).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![language, limit as i64], |row| {
            Ok(TenseStat {
                tense: row.get(0)?,
                total: row.get(1)?,
                correct: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| e.to_string())?);
    }
    Ok(results)
}

/// Save a completed conjugation exercise to the history table.
/// Returns the row ID so the frontend can later record the answer result.
fn save_conjugation_to_history(
    conn: &Connection,
    language: &str,
    exercise: &ConjugationExercise,
) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO conjugation_history (language_code, verb, tense, subject, sentence, answer, translation) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            language,
            exercise.verb,
            exercise.tense,
            exercise.subject,
            exercise.sentence,
            exercise.answer,
            exercise.translation
        ]
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

/// Generate a new conjugation exercise, seeded with history + weakness stats.
/// Returns (exercise, history_row_id) so the caller can later record the answer.
pub async fn process_conjugation_generation(
    db: Arc<Mutex<Connection>>,
    ai: Arc<dyn LlmProvider + Send + Sync>,
    language: String,
    model: String,
) -> Result<(ConjugationExercise, i64), String> {
    // 1. Fetch recent history + tense stats from DB
    let (previously_used, tense_stats, active_theme) = {
        let conn_arc = db.clone();
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        let limit = 10;
        let prev = get_recent_conjugations(&conn, &language, limit)?;
        let stats = get_tense_stats(&conn, &language, 1000)?;
        drop(conn); // Drop the lock before getting curriculum to avoid deadlock
        let curriculum = crate::services::curriculum::get_curriculum(conn_arc, language.clone())?;
        (prev, stats, curriculum.active_theme_id)
    };

    if !previously_used.is_empty() {
        println!(
            "[Conjugator] Loaded {} previous exercises to avoid repeats.",
            previously_used.len()
        );
    }
    if !tense_stats.is_empty() {
        println!("[Conjugator] Tense stats (last 1000 answered):");
        for s in &tense_stats {
            let pct = if s.total > 0 {
                (s.correct as f64 / s.total as f64) * 100.0
            } else {
                0.0
            };
            println!("  {} — {}/{} ({:.0}%)", s.tense, s.correct, s.total, pct);
        }
    }

    // 2. Generate a new exercise, passing history + weakness data + theme
    let mut exercise = ai
        .generate_conjugation_exercise(language.clone(), model, previously_used, tense_stats, active_theme)
        .await?;

    // Sanitize tense format (lowercase and remove anything in parenthesis)
    if let Some(idx) = exercise.tense.find('(') {
        exercise.tense = exercise.tense[..idx].trim().to_string();
    }
    exercise.tense = exercise.tense.to_lowercase();

    let exercise_clone = exercise.clone();
    let lang_clone = language.clone();

    // 3. Save to conjugation_history AND vocabulary in a single blocking task
    let row_id = tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;

        // Persist to conjugation history (answered_correct stays NULL until the user answers)
        let id = save_conjugation_to_history(&conn, &lang_clone, &exercise_clone)?;

        // Also save as vocabulary for SRS flashcards (existing behaviour)
        let target_text = format!("{} ({})", exercise_clone.verb, exercise_clone.tense);
        let native_text = format!("{} -> {}", exercise_clone.translation, exercise_clone.answer);

        conn.execute(
            "INSERT INTO vocabulary (language_code, target_text, native_text, is_character) VALUES (?1, ?2, ?3, 0)",
            (&lang_clone, &target_text, &native_text)
        ).map_err(|e| e.to_string())?;

        Ok::<i64, String>(id)
    }).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

    Ok((exercise, row_id))
}

/// Record whether the user answered a conjugation exercise correctly.
/// Updates the `answered_correct` column on the row inserted during generation.
pub async fn record_conjugation_result(
    db: Arc<Mutex<Connection>>,
    history_id: i64,
    correct: bool,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        conn.execute(
            "UPDATE conjugation_history SET answered_correct = ?1 WHERE id = ?2",
            rusqlite::params![correct, history_id],
        )
        .map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_all_tense_stats(
    db: Arc<Mutex<Connection>>,
    language: String,
) -> Result<Vec<TenseStat>, String> {
    let conn = db.lock().map_err(|_| "DB lock failed")?;
    get_tense_stats(&conn, &language, 1000)
}
