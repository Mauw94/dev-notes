use crate::{
    config::Config,
    utils::{
        fetcher::{self, Note},
        writer::FileWriter,
    },
};

#[tauri::command]
pub async fn write_to_file(text: String, file_name: String) {
    let config = Config::default();
    let file_writer = FileWriter::new();
    file_writer
        .write(file_name, config.files_folder, text)
        .unwrap();
}

#[tauri::command]
pub async fn fetch_all_notes() -> Vec<Note> {
    fetcher::fetch_all_notes().unwrap()
}

#[tauri::command]
pub async fn fetch_note_content(path: String) -> String {
    fetcher::fetch_note_content(path).unwrap()
}

#[tauri::command]
pub async fn fetch_note(file_name: String) -> Result<Note, String> {
    match fetcher::fetch_note(file_name) {
        Ok(note) => Ok(note),
        Err(()) => Err("Didn't find note".to_string()),
    }
}

#[tauri::command]
pub async fn fetch_app_dir() -> String {
    Config::default().app_folder
}

#[tauri::command]
pub async fn fetch_files_dir() -> String {
    Config::default().files_folder
}
