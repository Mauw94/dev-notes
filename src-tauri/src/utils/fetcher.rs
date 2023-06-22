use std::{
    fs::{self},
    io::Error,
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::cacher::get_cache;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub file_name: String,
    pub path: String,
    pub text: String,
    pub modified_time: SystemTime,
    pub has_due_date: bool,
    pub is_active: bool,
    pub is_completed: bool,
}

impl Note {
    pub fn new(file_name: &str, path: &str, text: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            path: path.to_string(),
            text: text.to_string(),
            modified_time: SystemTime::now(),
            has_due_date: false,
            is_active: true,
            is_completed: false
        }
    }
}

pub fn new_note(file_name: String, path: String, text: String) {
    let binding = get_cache();
    let mut cache = binding.lock().unwrap();
    match cache.new_note(file_name, path, text) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err),
    }
}

pub fn fetch_all_notes_from_cache() -> Result<Vec<Note>, Error> {
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

pub fn fetch_note_content_from_cache(file_name: String) -> String {
    let binding = get_cache();
    let cache = binding.lock().unwrap();
    let note = cache.get_note(file_name);
    match note {
        Some(n) => n.text,
        None => String::new(),
    }
}

pub fn fetch_note_from_cache(file_name: String) -> Result<Note, ()> {
    let binding = get_cache();
    let cache = binding.lock().unwrap();
    let note = cache.get_note(file_name);
    match note {
        Some(n) => Ok(n),
        None => Err(()),
    }
}

pub fn update_note_in_cache(file_name: String, path: String, text: String) {
    let cache = get_cache();
    let mut cache_lock = cache.lock().unwrap();
    cache_lock.update_cache(file_name, path, text);
}

// TODO fix
// fn get_cache_lock<'a>() -> std::sync::MutexGuard<'a, super::cacher::Cache> {
//     let cache_lock = get_cache().lock().unwrap();
//     cache_lock
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_all_notes_returns_ok() {
        let notes = fetch_all_notes_from_cache();
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
