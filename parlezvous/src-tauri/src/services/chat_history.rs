use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatHistoryMessage {
    pub id: i64,
    pub role: String,
    pub content: String,
    pub correction: Option<String>,
    #[serde(rename = "audioBase64")]
    pub audio_base64: Option<String>,
}

pub fn list(conn: &Connection) -> Result<Vec<ChatHistoryMessage>, String> {
    let mut stmt = conn
        .prepare("SELECT id, role, content, correction, audio_base64 FROM avatar_chat_history ORDER BY id ASC")
        .map_err(|error| error.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(ChatHistoryMessage {
                id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                correction: row.get(3)?,
                audio_base64: row.get(4)?,
            })
        })
        .map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| error.to_string())
}

pub fn insert(
    conn: &Connection,
    role: &str,
    content: &str,
    correction: Option<&str>,
    audio_base64: Option<&str>,
) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO avatar_chat_history (role, content, correction, audio_base64) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![role, content, correction, audio_base64],
    )
    .map_err(|error| error.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_user_correction(conn: &Connection, id: i64, correction: &str) -> Result<(), String> {
    let updated = conn
        .execute(
            "UPDATE avatar_chat_history SET correction = ?1 WHERE id = ?2 AND role = 'user'",
            rusqlite::params![correction, id],
        )
        .map_err(|error| error.to_string())?;
    if updated == 0 { return Err(format!("User chat message {id} was not found")); }
    Ok(())
}

pub fn clear(conn: &Connection) -> Result<(), String> {
    conn.execute("DELETE FROM avatar_chat_history", [])
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn database() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE avatar_chat_history (\n                id INTEGER PRIMARY KEY AUTOINCREMENT,\n                role TEXT NOT NULL, content TEXT NOT NULL, correction TEXT, audio_base64 TEXT\n            );",
        ).unwrap();
        conn
    }

    #[test]
    fn persists_corrections_on_the_existing_user_row() {
        let conn = database();
        let user_id = insert(&conn, "user", "je aller", None, None).unwrap();
        let assistant_id = insert(&conn, "assistant", "Essaie encore.", None, None).unwrap();
        update_user_correction(&conn, user_id, "je vais").unwrap();

        let history = list(&conn).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].id, user_id);
        assert_eq!(history[0].correction.as_deref(), Some("je vais"));
        assert!(update_user_correction(&conn, assistant_id, "invalid").is_err());

        clear(&conn).unwrap();
        assert!(list(&conn).unwrap().is_empty());
    }
}
