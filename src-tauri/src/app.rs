use std::{fs::create_dir_all, io::Error, path::Path};

use crate::{
    config::{self, Config},
    utils::writer::FileWriter,
};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn setup(&self) -> Result<(), Error> {
        create_dir_all(self.config.files_folder.clone())?;
        if !Path::new(&config::Config::test_file_path()).exists() {
            let file_writer = FileWriter::new();
            match file_writer.write(
                &String::from("test_file.txt"),
                &self.config.files_folder,
                &String::from("test_file_123"),
            ) {
                Ok(()) => {}
                Err(err) => eprintln!("{}", err),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_app() {
        let app = App::new(Config::default());
        let setup_result = app.setup();
        assert_eq!(setup_result.is_ok(), true);
    }
}
