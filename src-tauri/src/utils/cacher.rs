use crate::config::Config;
use lazy_static::lazy_static;
use std::{
    fs,
    io::Error,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use super::{fetcher::Note, writer::FileWriter};

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

        if !Path::new(config.files_folder.as_str()).exists() {
            panic!("Config is not correct");
        }

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
                                modified_time: entry.metadata().unwrap().modified().unwrap(),
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

        self.initialized = true;
        self.expires_in = Duration::from_secs_f32(60.0 * 60.0);

        Ok(())
    }

    pub fn get_all(&self) -> &Vec<Note> {
        self.notes.as_ref()
    }

    pub fn get_note(&self, file_name: String) -> Option<Note> {
        let found_note = self.notes.iter().find(|n| n.file_name == file_name);
        match found_note {
            Some(note) => Some(note.clone()),
            None => None,
        }
    }

    pub fn new_note(&mut self, file_name: String, path: String, text: String) -> Result<(), Error> {
        let file_writer = FileWriter::new();
        match file_writer.write(file_name.as_str(), &path, &text) {
            Ok(()) => {
                let note = Note::new(&file_name, &path, &text);
                self.notes.push(note);
            }
            Err(err) => eprintln!("{}", err),
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_initialize_contains_test_file() {
        let cache = get_cache();
        let mut cache_lock = cache.lock().unwrap();
        cache_lock.initialize().unwrap();
        let notes = cache_lock.get_all();
        assert!(notes.len() > 0);
    }

    #[test]
    fn add_new_note() {
        let cache = get_cache();
        let mut cache_lock = cache.lock().unwrap();
        let first_notes_count = cache_lock.get_all().len();
        match cache_lock.new_note(
            String::from("test123.txt"),
            String::from("random stuff"),
            String::from("random text"),
        ) {
            Ok(()) => {}
            Err(err) => eprint!("{}", err),
        }

        assert_eq!(first_notes_count + 1, cache_lock.get_all().len());
    }
}
