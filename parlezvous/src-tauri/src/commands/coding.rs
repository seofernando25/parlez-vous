use crate::{AppState};
use tauri::State;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CodingQueueItem {
    pub question_type: String,
    pub question_data: String,
}

#[tauri::command]
pub(crate) async fn generate_coding_puzzle(
    state: State<'_, AppState>,
    language: String,
    model: String,
    theme: String,
    puzzle_type: String,
) -> Result<String, String> {
    let db = state.db.clone();
    let previously_used = tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        let mut stmt = conn.prepare("SELECT question_data FROM coding_questions_history ORDER BY id DESC LIMIT 3").map_err(|e| e.to_string())?;
        let rows = stmt.query_map([], |row| row.get(0)).map_err(|e| e.to_string())?;
        let mut used = Vec::new();
        for r in rows {
            if let Ok(data) = r {
                used.push(data);
            }
        }
        Ok::<Vec<String>, String>(used)
    }).await.map_err(|e| e.to_string())??;

    let ai = state.router.provider_for(&model)?;
    let result = ai.generate_coding_puzzle(language, model, theme, puzzle_type.clone(), previously_used).await?;

    let db2 = state.db.clone();
    let res_clone = result.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db2.lock().map_err(|_| "DB lock failed")?;
        conn.execute(
            "INSERT INTO coding_questions_history (question_type, question_data) VALUES (?1, ?2)",
            [&puzzle_type, &res_clone]
        ).map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    Ok(result)
}

#[tauri::command]
pub(crate) async fn save_coding_queue(
    state: State<'_, AppState>,
    queue: Vec<CodingQueueItem>,
) -> Result<(), String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let mut conn = db.lock().map_err(|_| "DB lock failed")?;
        let tx = conn.transaction().map_err(|e| e.to_string())?;

        tx.execute("DELETE FROM coding_questions_queue", []).map_err(|e| e.to_string())?;

        {
            let mut stmt = tx.prepare("INSERT INTO coding_questions_queue (question_type, question_data) VALUES (?1, ?2)").map_err(|e| e.to_string())?;
            for item in queue {
                stmt.execute([&item.question_type, &item.question_data]).map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(())
    }).await.map_err(|e| format!("Task failed: {}", e))?
}

#[tauri::command]
pub(crate) async fn load_coding_queue(
    state: State<'_, AppState>,
) -> Result<Vec<CodingQueueItem>, String> {
    let db = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        let mut stmt = conn.prepare("SELECT question_type, question_data FROM coding_questions_queue ORDER BY id ASC").map_err(|e| e.to_string())?;

        let iter = stmt.query_map([], |row| {
            Ok(CodingQueueItem {
                question_type: row.get(0)?,
                question_data: row.get(1)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut queue = Vec::new();
        for item in iter {
            queue.push(item.map_err(|e| e.to_string())?);
        }
        Ok(queue)
    }).await.map_err(|e| format!("Task failed: {}", e))?
}
