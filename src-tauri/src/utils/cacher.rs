use std::{fs, io::Error, path::Path, time::Duration};

use crate::config::Config;

use super::fetcher::Note;

pub struct Cache {
    pub notes: Vec<Note>,
    pub initialized: bool,
    pub expires_in: Duration,
}

pub trait Caching {
    fn initialize(&self) -> Result<(), Error>;
}

impl Cache {
    pub fn new() -> Self {
        Self {
            notes: vec![],
            initialized: false,
            expires_in: Duration::from_secs_f32(60.0 * 60.0),
        }
    }

    pub fn cache(&mut self, n: Vec<Note>) {
        self.notes = n;
    }

    pub fn get_cache(&self) -> Vec<Note> {
        self.notes.clone()
    }
}

impl Caching for Cache {
    fn initialize(&self) -> Result<(), Error> {
        println!("initializing");
        let mut notes: Vec<Note> = vec![];
        let config = Config::default();
        if Path::new(config.files_folder.as_str()).exists() {
            println!("files folder exists");
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
                                notes.push(n);
                            }
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }
}
