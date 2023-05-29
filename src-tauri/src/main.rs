// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod commands;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    // TODO intialize cache and read from saved files

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::write_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
