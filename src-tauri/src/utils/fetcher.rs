use std::{
    fs::{self},
    io::Error,
};

use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::cacher::get_cache;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub file_name: String,
    pub path: String,
    pub text: String,
}

impl Note {
    pub fn new(file_name: String, path: String, text: String) -> Self {
        Self {
            file_name,
            path,
            text,
        }
    }
}

pub fn fetch_all_notes() -> Result<Vec<Note>, Error> {
    let cfg = Config::default();
    let binding = get_cache();
    let cache = binding.lock().unwrap();
    let notes = cache.get_all().clone();

    Ok(notes
        .into_iter()
        .filter(|f| f.file_name != cfg.test_file_name)
        .collect())
}

pub fn fetch_note_content(path: String) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn fetch_note(file_name: String) -> Result<Note, ()> {
    let cache = get_cache();
    let cache_lock = cache.lock().unwrap();
    let note = cache_lock.get_note(file_name);
    match note {
        Some(n) => Ok(n),
        None => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_all_notes_returns_ok() {
        let notes = fetch_all_notes();
        assert!(notes.is_ok());
    }

    #[test]
    fn fetch_note_content_returns_ok() {
        let note = fetch_note_content(Config::test_file_path());
        assert!(note.is_ok());
    }

    #[test]
    fn fetch_note_content_bad_path_returns_err() {
        let note = fetch_note_content(String::from("this path does not exist"));
        assert!(note.is_err());
    }
}
