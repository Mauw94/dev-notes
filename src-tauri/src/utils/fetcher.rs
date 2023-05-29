use std::{fs, io::Error};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub file_name: String,
    pub path: String,
}

impl Note {
    pub fn new(file_name: String, path: String) -> Self {
        Self { file_name, path }
    }
}

pub fn fetch_all_notes() -> Result<Vec<Note>, Error> {
    let mut notes: Vec<Note> = Vec::new();
    let cfg = Config::default();
    let paths = fs::read_dir(cfg.files_folder)?;

    for path in paths {
        let file_name = path
            .as_ref()
            .unwrap()
            .file_name()
            .to_os_string()
            .into_string()
            .unwrap();
        let file_path = path.as_ref().unwrap().path().display().to_string();
        let note = Note::new(file_name, file_path.to_string());
        notes.push(note);
    }

    Ok(notes)
}
