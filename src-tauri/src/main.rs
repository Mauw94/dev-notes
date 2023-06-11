// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use app::App;
use config::Config;
use utils::cacher::get_cache;

mod app;
mod commands;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    let app = App::new(Config::default());
    app.setup().unwrap();

    let cache = get_cache();
    {
        let mut cache_lock = cache.lock().unwrap();
        cache_lock.initialize().unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::write_to_file,
            commands::fetch_all_notes_from_cache,
            commands::fetch_note_content,
            commands::fetch_note,
            commands::fetch_app_dir,
            commands::fetch_files_dir,
            commands::fetch_note_content_from_cache,
            commands::update_note_in_cache,
            commands::auth_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
