// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use app::App;
use config::Config;
use utils::{cacher::{Cache, Caching}, fetcher::Note};

mod app;
mod commands;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    // TODO build cache from files dir on load
    // TODO intialize cache and read from saved files
    let note = Note {
        file_name: String::from("test1.txt"),
        path: String::from("asdasd"),
        text: String::from("asd"),
    };
    let mut notes: Vec<Note> = vec![];
    notes.push(note);

    let mut cache = Cache::new();
    cache.initialize();
    cache.cache(notes);

    let app = App::new(Config::default());
    app.setup().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::write_to_file,
            commands::fetch_all_notes,
            commands::fetch_note_content,
            commands::fetch_app_dir,
            commands::fetch_files_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
