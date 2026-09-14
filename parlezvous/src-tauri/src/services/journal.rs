use crate::ai::{GradingVariables, JournalProvider, JournalResponse, JournalVariables};
use crate::db::VocabId;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

// The returned DbVocabItem structure is passed back so Tauri can emit it
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DbVocabItem {
    pub id: VocabId,
    pub target_text: String,
    pub native_text: String,
    pub is_character: bool,
}

pub async fn process_journal_generation<P>(
    db: Arc<Mutex<Connection>>,
    ai: Arc<P>,
    variables: JournalVariables,
) -> Result<(JournalResponse, Vec<DbVocabItem>), String>
where
    P: JournalProvider + Send + Sync + ?Sized,
{
    let mood = variables.mood.clone();
    let weather = variables.weather.clone();
    let activity = variables.activity.clone();
    let language = variables.language.clone();

    let response = ai.generate_guided_journal(variables).await?;
    let response_clone = response.clone();

    let db_vocab_chips = tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        
        conn.execute(
            "INSERT INTO journal_entries (language_code, date, mood_input, weather_input, activity_input, generated_target_text, native_translation) VALUES (?1, CURRENT_TIMESTAMP, ?2, ?3, ?4, ?5, ?6)",
            (&language, &mood, &weather, &activity, &response_clone.generated_target_text, &response_clone.native_translation)
        ).map_err(|e| e.to_string())?;
        
        let journal_id = conn.last_insert_rowid();
        let mut chips = vec![];

        for vocab in &response_clone.vocabulary {
            conn.execute(
                "INSERT INTO vocabulary (language_code, target_text, native_text, is_character) VALUES (?1, ?2, ?3, 0)",
                (&language, &vocab.target_text, &vocab.native_text)
            ).map_err(|e| e.to_string())?;
            
            let vocab_id = VocabId(conn.last_insert_rowid());

            conn.execute(
                "INSERT INTO journal_vocabulary (journal_id, vocab_id) VALUES (?1, ?2)",
                (journal_id, vocab_id)
            ).map_err(|e| e.to_string())?;

            chips.push(DbVocabItem {
                id: vocab_id,
                target_text: vocab.target_text.clone(),
                native_text: vocab.native_text.clone(),
                is_character: false,
            });
        }
        
        Ok::<Vec<DbVocabItem>, String>(chips)
    }).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

    Ok((response, db_vocab_chips))
}

pub async fn process_journal_grading<P>(
    db: Arc<Mutex<Connection>>,
    ai: Arc<P>,
    variables: GradingVariables,
) -> Result<(JournalResponse, Vec<DbVocabItem>), String>
where
    P: JournalProvider + Send + Sync + ?Sized,
{
    let language = variables.language.clone();

    let response = ai.grade_custom_journal(variables).await?;
    let response_clone = response.clone();

    let db_vocab_chips = tokio::task::spawn_blocking(move || {
        let conn = db.lock().map_err(|_| "DB lock failed")?;
        
        conn.execute(
            "INSERT INTO journal_entries (language_code, date, mood_input, weather_input, activity_input, generated_target_text, native_translation) VALUES (?1, CURRENT_TIMESTAMP, ?2, ?3, ?4, ?5, ?6)",
            (&language, &"Custom".to_string(), &"Custom".to_string(), &"Custom".to_string(), &response_clone.generated_target_text, &response_clone.native_translation)
        ).map_err(|e| e.to_string())?;
        
        let journal_id = conn.last_insert_rowid();
        let mut chips = vec![];

        for vocab in &response_clone.vocabulary {
            conn.execute(
                "INSERT INTO vocabulary (language_code, target_text, native_text, is_character) VALUES (?1, ?2, ?3, 0)",
                (&language, &vocab.target_text, &vocab.native_text)
            ).map_err(|e| e.to_string())?;
            
            let vocab_id = VocabId(conn.last_insert_rowid());

            conn.execute(
                "INSERT INTO journal_vocabulary (journal_id, vocab_id) VALUES (?1, ?2)",
                (journal_id, vocab_id)
            ).map_err(|e| e.to_string())?;

            chips.push(DbVocabItem {
                id: vocab_id,
                target_text: vocab.target_text.clone(),
                native_text: vocab.native_text.clone(),
                is_character: false,
            });
        }
        
        Ok::<Vec<DbVocabItem>, String>(chips)
    }).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

    Ok((response, db_vocab_chips))
}
