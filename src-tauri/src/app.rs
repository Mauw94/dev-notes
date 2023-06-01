use std::{fs::create_dir_all, io::Error, path::Path};

use crate::{
    config::{self, Config},
    utils::writer::{FileWriter}};

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
                let file_writer = FileWriter::new();
                match file_writer.write(
                    String::from("test_file.txt"),
                    self.config.files_folder.to_owned(),
                    String::from("test_file_123"),
                ) {
                    Ok(()) => {}
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Ok(())
    }
}
