pub mod ai;
pub(crate) mod commands;
pub mod db;
pub mod model;
pub mod services;

use ai::router::AiRouter;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub(crate) struct AppState {
    pub(crate) db: Arc<Mutex<Connection>>,
    pub(crate) router: AiRouter,
    pub(crate) skill_level: std::sync::RwLock<String>,
    pub(crate) conjugation_task: tokio::sync::Mutex<Option<tokio::task::AbortHandle>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_litert::init())
        .plugin(tauri_plugin_supertonic::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let conn = db::init_db(app.handle()).map_err(|e| {
                eprintln!("❌ CRITICAL: Failed to initialize database: {}", e);
                e
            })?;
            let db_arc = Arc::new(Mutex::new(conn));
            let initial_skill_level = crate::services::profile::get_skill_level(db_arc.clone())
                .unwrap_or_else(|_| "Beginner".to_string());
                
            let router = AiRouter::new(db_arc.clone(), app.handle().clone());

            app.manage(AppState {
                db: db_arc,
                router,
                skill_level: std::sync::RwLock::new(initial_skill_level),
                conjugation_task: tokio::sync::Mutex::new(None),
            });

            #[cfg(all(target_os = "linux", not(target_os = "android")))]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.with_webview(|webview| {
                        use webkit2gtk::PermissionRequestExt;
                        use webkit2gtk::WebViewExt;
                        webview
                            .inner()
                            .connect_permission_request(move |_, request| {
                                request.allow();
                                true
                            });
                    });
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::check_ai_health,
            commands::system::list_ai_models,
            commands::profile::add_active_seconds,
            commands::profile::get_curriculum,
            commands::profile::add_time_xp,
            commands::profile::set_active_theme,
            commands::profile::get_profile,
            commands::profile::set_profile_display_name,
            commands::journal::generate_journal,
            commands::avatar::chat_with_avatar,
            commands::journal::add_to_srs,
            commands::journal::get_all_vocabulary,
            commands::system::get_settings,
            commands::system::update_settings,
            commands::handwriting::infer_character,
            commands::handwriting::get_all_jamo,
            commands::handwriting::get_alphabet_letters,
            commands::conjugation::generate_conjugation_exercise,
            commands::conjugation::record_conjugation_result,
            commands::journal::get_journal_entries,
            commands::textbooks::upload_and_ingest_textbook,
            commands::profile::get_user_skill_level,
            commands::profile::set_user_skill_level,
            commands::textbooks::list_textbooks,
            commands::textbooks::get_textbook_dir,
            commands::journal::grade_journal,
            commands::speech::generate_tts_audio,
            commands::system::check_tokenizer_exists,
            commands::system::download_tokenizer,
            commands::system::delete_tokenizer,
            commands::conjugation::get_all_tense_stats,
            commands::conjugation::cancel_conjugation_generation,
            commands::coding::generate_coding_puzzle,
            commands::coding::save_coding_queue,
            commands::coding::load_coding_queue,
            commands::language_game::generate_language_puzzle,
            commands::language_game::save_language_queue,
            commands::language_game::load_language_queue,
            commands::avatar::get_chat_history,
            commands::avatar::save_chat_message,
            commands::avatar::update_chat_message_correction,
            commands::avatar::clear_chat_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
