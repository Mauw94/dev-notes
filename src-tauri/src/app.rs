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
        self.setup_files_dir()?;
        self.setup_meta_files_dir()?;
        Ok(())
    }

    fn setup_files_dir(&self) -> Result<(), Error> {
        create_dir_all(self.config.files_folder.clone())?;

        // write a test file to the folder, if it doensn't yet exist
        if !Path::new(&config::Config::test_file_path()).exists() {
            let file_writer = FileWriter::new();
            match file_writer.write(
                &self.config.test_file_name,
                &self.config.files_folder,
                "test content 123",
            ) {
                Ok(()) => {}
                Err(err) => eprintln!("{}", err),
            }
        }
        Ok(())
    }
 
    fn setup_meta_files_dir(&self) -> Result<(), Error> {
        create_dir_all(self.config.note_meta_data_folder.clone())?;

        // write a test file to the folder, if it doensn't yet exist
        if !Path::new(&config::Config::test_meta_file_path()).exists() {
            let file_writer = FileWriter::new();
            match file_writer.write(
                &self.config.test_file_name,
                &self.config.note_meta_data_folder,
                // TODO: get info from the file
                "has_due_data: false\nis_active: true\nis_completed: false",
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
