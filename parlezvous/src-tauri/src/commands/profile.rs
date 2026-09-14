use crate::AppState;
use tauri::State;

#[tauri::command]
pub(crate) async fn get_user_skill_level(state: State<'_, AppState>) -> Result<String, String> {
    let skill = state
        .skill_level
        .read()
        .map_err(|_| "Failed to lock skill_level")?
        .clone();
    Ok(skill)
}

#[tauri::command]
pub(crate) async fn set_user_skill_level(state: State<'_, AppState>, level: String) -> Result<(), String> {
    crate::services::profile::update_skill_level(state.db.clone(), level.clone())?;
    let mut skill = state
        .skill_level
        .write()
        .map_err(|_| "Failed to lock skill_level")?;
    *skill = level;
    Ok(())
}

#[tauri::command]
pub(crate) async fn add_active_seconds(state: State<'_, AppState>, seconds: i32) -> Result<(), String> {
    crate::services::profile::add_active_seconds(state.db.clone(), seconds)
}

#[tauri::command]
pub(crate) async fn get_curriculum(
    state: State<'_, AppState>,
    language: String,
) -> Result<crate::services::curriculum::LanguageCurriculum, String> {
    crate::services::curriculum::get_curriculum(state.db.clone(), language)
}

#[tauri::command]
pub(crate) async fn add_time_xp(
    state: State<'_, AppState>,
    language: String,
    seconds: i32,
) -> Result<(), String> {
    crate::services::curriculum::add_time_xp(state.db.clone(), language, seconds)
}

#[tauri::command]
pub(crate) async fn set_active_theme(
    state: State<'_, AppState>,
    language: String,
    theme_id: String,
) -> Result<(), String> {
    crate::services::curriculum::set_active_theme(state.db.clone(), language, theme_id)
}

#[tauri::command]
pub(crate) async fn get_profile(
    state: State<'_, AppState>,
) -> Result<crate::services::profile::UserProfile, String> {
    crate::services::profile::get_profile(state.db.clone())
}


#[tauri::command]
pub(crate) async fn set_profile_display_name(
    state: State<'_, AppState>,
    display_name: String,
) -> Result<(), String> {
    crate::services::profile::update_display_name(state.db.clone(), display_name)
}
