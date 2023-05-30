use crate::{
    config::Config,
    utils::{
        fetcher::{self, Note},
        writer::{FileWriter, Writer},
    },
};

#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn write_to_file(text: String, file_name: String) {
    let config = Config::default();
    let file_writer = FileWriter::new(config.files_folder);
    match file_writer.write(text, file_name) {
        Ok(_) => {
            println!("{}", "Successfully written to file.");
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
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
pub async fn fetch_app_dir() -> String {
    let config = Config::default();
    config.app_folder
}

#[tauri::command]
pub async fn fetch_files_dir() -> String {
    let config = Config::default();
    config.files_folder
}
