use crate::config::Config;
use lazy_static::lazy_static;
use std::{
    fs,
    io::Error,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use super::fetcher::Note;

pub struct Cache {
    pub notes: Vec<Note>,
    pub initialized: bool,
    pub expires_in: Duration,
}

lazy_static! {
    pub static ref GLOBAL_CACHE: Arc<Mutex<Cache>> = Arc::new(Mutex::new(Cache::new()));
}

pub fn get_cache() -> Arc<Mutex<Cache>> {
    GLOBAL_CACHE.clone()
}

impl Cache {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            initialized: false,
            expires_in: Duration::ZERO,
        }
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        let config = Config::default();
        if Path::new(config.files_folder.as_str()).exists() {
            for entry in fs::read_dir(config.files_folder)? {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if let Some(ext) = path.extension() {
                            if ext.to_string_lossy().ends_with("txt") {
                                let content = fs::read_to_string(entry.path())?;
                                let n = Note {
                                    file_name: entry.file_name().to_string_lossy().to_string(),
                                    path: entry.path().to_string_lossy().to_string(),
                                    text: content,
                                };
                                self.notes.push(n);
                            }
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }

        self.initialized = true;
        self.expires_in = Duration::from_secs_f32(60.0 * 60.0);

        Ok(())
    }

    pub fn get_all(&self) -> &Vec<Note> {
        self.notes.as_ref()
    }

    pub fn new_note(&mut self, file_name: String, path: String, text: String) {
        let note = Note::new(file_name, path, text);
        self.notes.push(note);
    }

    pub fn update_cache(&mut self, file_name: String, path: String, text: String) {
        let note_index = self.notes.iter().position(|n| n.file_name == file_name);
        match note_index {
            Some(idx) => {
                self.notes[idx].file_name = file_name;
                self.notes[idx].path = path;
                self.notes[idx].text = text;
            }
            None => {}
        }
    }
}
