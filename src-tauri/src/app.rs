use std::{fs::create_dir_all, io::Error, path::Path};

use crate::{
    config::{self, Config},
    utils::writer::{FileWriter, Writer},
};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn setup(&self) -> Result<(), Error> {
        let create_dir = create_dir_all(self.config.files_folder.clone());
        if create_dir.is_ok() {
            if !Path::new(&config::Config::test_file_path()).exists() {
                let file_writer = FileWriter::new(self.config.files_folder.clone());
                match file_writer
                    .write(String::from("test_file_123"), String::from("test_file.txt"))
                {
                    Ok(()) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Ok(())
    }
}
