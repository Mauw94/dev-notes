use crate::{
    config::Config,
    utils::{
        auth,
        fetcher::{self, Note},
        writer::FileWriter,
    },
};

#[tauri::command]
pub async fn write_to_file(text: String, file_name: String) {
    let config = Config::default();
    let file_writer = FileWriter::new();
    file_writer
        .write(&file_name, &config.files_folder, &text)
        .unwrap();
}

#[tauri::command]
pub async fn new_note(file_name: String, path: String, text: String) {
    fetcher::new_note(file_name, path, text)
}

#[tauri::command]
pub async fn fetch_all_notes_from_cache() -> Vec<Note> {
    fetcher::fetch_all_notes_from_cache().unwrap()
}

#[tauri::command]
pub async fn fetch_note_content(path: String) -> String {
    fetcher::fetch_note_content(path).unwrap()
}

#[tauri::command]
pub async fn fetch_note(file_name: String) -> Result<Note, String> {
    match fetcher::fetch_note_from_cache(file_name) {
        Ok(note) => Ok(note),
        Err(()) => Err("Didn't find note".to_string()),
    }
}

#[tauri::command]
pub async fn update_note_in_cache(file_name: String, path: String, text: String) {
    fetcher::update_note_in_cache(file_name, path, text)
}

#[tauri::command]
pub async fn fetch_app_dir() -> String {
    Config::default().app_folder
}

#[tauri::command]
pub async fn fetch_files_dir() -> String {
    Config::default().files_folder
}

#[tauri::command]
pub async fn fetch_note_content_from_cache(file_name: String) -> String {
    fetcher::fetch_note_content_from_cache(file_name)
}

#[tauri::command]
pub async fn auth_user(name: String, pass: String) -> bool {
    auth::login(name, pass).unwrap()
}
