use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
    path::Path,
};

use super::cacher::get_cache;
pub(crate) struct FileWriter {}

impl FileWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, file_name: String, folder: String, text: String) -> Result<(), Error> {
        let mut file_path = folder.to_owned();
        let file_n = file_name.to_owned();
        file_path.push_str(&file_n);
        let mut f: File;

        if Path::new(&file_path).exists() {
            f = OpenOptions::new()
                .create_new(false)
                .write(true)
                .append(true)
                .open(file_path)?;

            f.write(text.as_bytes())?;
            self.update_cache(file_name, folder, text);
        } else {
            f = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(file_path)?;

            f.write(text.as_bytes())?;
            self.add_to_cache(file_name, folder, text);
        }

        Ok(())
    }

    fn add_to_cache(&self, file_name: String, path: String, text: String) {
        let cache = get_cache();
        let mut cache_lock = cache.lock().unwrap();
        cache_lock.new_note(file_name, path, text);
    }

    fn update_cache(&self, file_name: String, path: String, text: String) {
        let cache = get_cache();
        let mut cache_lock = cache.lock().unwrap();
        cache_lock.update_cache(file_name, path, text);
    }
}
