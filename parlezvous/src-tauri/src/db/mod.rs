mod ids;
mod schema;

pub use ids::{JournalId, VocabId};
pub use schema::SCHEMA_V1;

use rusqlite::ffi::sqlite3_auto_extension;
use rusqlite::Connection;
use schema::{DB_VERSION_NUM, SCHEMA_V2, SCHEMA_V3, SCHEMA_V4, SCHEMA_V5, SCHEMA_V6, SCHEMA_V7, SCHEMA_V8, SCHEMA_V9};
use sqlite_vec::sqlite3_vec_init;
use tauri::Manager;

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|error| format!("Failed to resolve app_data_dir: {error}"))?;
    eprintln!("📂 DEBUG: app_data_dir resolved to: {:?}", app_dir);

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)
            .map_err(|error| format!("Failed to create base app_data_dir {:?}: {error}", app_dir))?;
    }
    let db_path = app_dir.join("sqlite.db");
    eprintln!("📂 DEBUG: Opening database at: {:?}", db_path);

    unsafe { sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ()))); }
    let conn = Connection::open(&db_path)
        .map_err(|error| format!("Failed to open database at {:?}: {error}", db_path))?;

    apply_migrations(&conn)?;
    Ok(conn)
}

fn apply_migrations(conn: &Connection) -> Result<(), String> {
    let mut user_version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    if user_version == 0 {
        let has_settings: i32 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='settings'", [], |row| row.get(0)
        ).unwrap_or(0);
        if has_settings > 0 {
            conn.pragma_update(None, "user_version", 1).map_err(|error| error.to_string())?;
            user_version = 1;
        }
    }

    let schemas = [SCHEMA_V1, SCHEMA_V2, SCHEMA_V3, SCHEMA_V4, SCHEMA_V5, SCHEMA_V6, SCHEMA_V7, SCHEMA_V8, SCHEMA_V9];
    for (index, schema) in schemas.iter().enumerate().take(DB_VERSION_NUM) {
        if user_version != index as i32 { continue; }
        conn.execute_batch(schema).map_err(|error| error.to_string())?;
        conn.pragma_update(None, "user_version", (index + 1) as i32).map_err(|error| error.to_string())?;
        user_version = (index + 1) as i32;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::apply_migrations;
    use rusqlite::Connection;

    #[test]
    fn upgrades_v7_through_current_profile_and_tts_defaults() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE settings (id INTEGER PRIMARY KEY CHECK (id = 1));
             INSERT INTO settings (id) VALUES (1);
             CREATE TABLE user_profile (id INTEGER PRIMARY KEY CHECK (id = 1), skill_level TEXT NOT NULL DEFAULT 'Beginner', tier INTEGER NOT NULL DEFAULT 1, active_seconds INTEGER NOT NULL DEFAULT 0);
             INSERT INTO user_profile (id) VALUES (1);
             PRAGMA user_version = 7;",
        ).unwrap();

        apply_migrations(&conn).unwrap();
        assert_eq!(conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i32>(0)).unwrap(), 9);
        let provider: String = conn.query_row("SELECT tts_provider FROM settings WHERE id = 1", [], |row| row.get(0)).unwrap();
        assert_eq!(provider, "auto");
        let name: String = conn.query_row("SELECT display_name FROM user_profile WHERE id = 1", [], |row| row.get(0)).unwrap_or_default();
        assert_eq!(name, "");
        apply_migrations(&conn).unwrap();
    }
}
