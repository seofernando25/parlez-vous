mod ids;
mod schema;

pub use ids::{JournalId, VocabId};
pub use schema::SCHEMA_CURRENT;

use rusqlite::ffi::sqlite3_auto_extension;
use rusqlite::Connection;
use schema::SCHEMA_VERSION;
use sqlite_vec::sqlite3_vec_init;
use std::path::{Path, PathBuf};
use tauri::Manager;

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;
    std::fs::create_dir_all(&app_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    unsafe { sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ()))); }
    let db_path = app_dir.join("sqlite.db");
    prepare_greenfield_database(&db_path)?;
    let conn = Connection::open(&db_path)
        .map_err(|error| format!("Failed to open database: {error}"))?;
    ensure_current_schema(&conn)?;
    Ok(conn)
}

fn prepare_greenfield_database(path: &Path) -> Result<(), String> {
    if !path.exists() || path.metadata().map(|meta| meta.len() == 0).unwrap_or(false) {
        return Ok(());
    }
    let conn = Connection::open(path).map_err(|error| error.to_string())?;
    if schema_is_current(&conn) {
        return Ok(());
    }
    drop(conn);
    let backup = legacy_backup_path(path);
    std::fs::rename(path, &backup)
        .map_err(|error| format!("Failed to back up legacy database: {error}"))?;
    for suffix in ["-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{}", path.display(), suffix));
        if sidecar.exists() {
            let backup_sidecar = PathBuf::from(format!("{}{}", backup.display(), suffix));
            let _ = std::fs::rename(sidecar, backup_sidecar);
        }
    }
    eprintln!("Backed up pre-greenfield database to {}", backup.display());
    Ok(())
}

fn legacy_backup_path(path: &Path) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    path.with_file_name(format!("sqlite.legacy-{timestamp}.db"))
}

fn schema_is_current(conn: &Connection) -> bool {
    conn.query_row(
        "SELECT value FROM app_meta WHERE key = 'schema_version'",
        [],
        |row| row.get::<_, String>(0),
    )
    .ok()
    .and_then(|value| value.parse::<i64>().ok()) == Some(SCHEMA_VERSION)
}

fn ensure_current_schema(conn: &Connection) -> Result<(), String> {
    if schema_is_current(conn) { return Ok(()); }
    conn.execute_batch(schema::SCHEMA_CURRENT).map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ensure_current_schema, prepare_greenfield_database, schema_is_current};
    use rusqlite::Connection;

    #[test]
    fn creates_the_canonical_schema_without_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        ensure_current_schema(&conn).unwrap();
        assert!(schema_is_current(&conn));
        let provider: String = conn.query_row(
            "SELECT ai_provider FROM settings WHERE id = 1", [], |row| row.get(0)
        ).unwrap();
        assert_eq!(provider, "managed");
        let tone: String = conn.query_row(
            "SELECT tutor_tone FROM settings WHERE id = 1", [], |row| row.get(0)
        ).unwrap();
        assert_eq!(tone, "balanced");
        let columns: i64 = conn.query_row(
            "SELECT count(*) FROM pragma_table_info('document_chunks') WHERE name = 'embedding_dimensions'",
            [], |row| row.get(0)
        ).unwrap();
        assert_eq!(columns, 1);
        let legacy_vec_table: i64 = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='vec_chunks'", [], |row| row.get(0)
        ).unwrap();
        assert_eq!(legacy_vec_table, 0);
    }

    #[test]
    fn backs_up_an_incompatible_database_instead_of_migrating_it() {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("parlezvous-db-{stamp}"));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("sqlite.db");
        let old = Connection::open(&path).unwrap();
        old.execute_batch("CREATE TABLE legacy_only (id INTEGER PRIMARY KEY); PRAGMA user_version=10;").unwrap();
        drop(old);

        prepare_greenfield_database(&path).unwrap();
        assert!(!path.exists());
        let backups: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten()
            .filter(|entry| entry.file_name().to_string_lossy().starts_with("sqlite.legacy-"))
            .collect();
        assert_eq!(backups.len(), 1);

        let current = Connection::open(&path).unwrap();
        ensure_current_schema(&current).unwrap();
        assert!(schema_is_current(&current));
        drop(current);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
