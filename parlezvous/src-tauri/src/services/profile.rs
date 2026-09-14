use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn get_skill_level(db: Arc<Mutex<Connection>>) -> Result<String, String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let level: String = conn
        .query_row(
            "SELECT skill_level FROM user_profile WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "Beginner".to_string());
    Ok(level)
}

pub fn update_skill_level(db: Arc<Mutex<Connection>>, level: String) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    conn.execute(
        "UPDATE user_profile SET skill_level = ?1 WHERE id = 1",
        rusqlite::params![level],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(serde::Serialize)]
pub struct UserProfile {
    pub skill_level: String,
    pub tier: i32,
    pub active_seconds: i32,
    pub display_name: String,
}

pub fn get_profile(db: Arc<Mutex<Connection>>) -> Result<UserProfile, String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let profile = conn
        .query_row(
            "SELECT skill_level, tier, active_seconds, display_name FROM user_profile WHERE id = 1",
            [],
            |row| {
                Ok(UserProfile {
                    skill_level: row.get(0)?,
                    tier: row.get(1)?,
                    active_seconds: row.get(2)?,
                    display_name: row.get::<_, String>(3).unwrap_or_default(),
                })
            },
        )
        .unwrap_or_else(|_| UserProfile {
            skill_level: "Beginner".to_string(),
            tier: 1,
            active_seconds: 0,
            display_name: String::new(),
        });
    Ok(UserProfile {
        display_name: if profile.display_name.trim().is_empty() { default_display_name() } else { profile.display_name },
        ..profile
    })
}

pub fn update_display_name(db: Arc<Mutex<Connection>>, display_name: String) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    conn.execute("UPDATE user_profile SET display_name = ?1 WHERE id = 1", rusqlite::params![display_name.trim()])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn default_display_name() -> String {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    { return "Learner".to_string(); }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        for key in ["USER", "LOGNAME", "USERNAME"] {
            if let Ok(value) = std::env::var(key) {
                let value = value.trim();
                if !value.is_empty() { return value.to_string(); }
            }
        }
        "Learner".to_string()
    }
}

pub fn add_active_seconds(db: Arc<Mutex<Connection>>, seconds: i32) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    conn.execute(
        "UPDATE user_profile SET active_seconds = active_seconds + ?1 WHERE id = 1",
        rusqlite::params![seconds],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
