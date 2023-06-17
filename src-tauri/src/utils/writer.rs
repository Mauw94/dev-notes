use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
    path::Path,
};

pub(crate) struct FileWriter {}

impl FileWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, file_name: &str, folder: &str, text: &str) -> Result<(), Error> {
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
        } else {
            f = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(file_path)?;

            f.write(text.as_bytes())?;
        }

        Ok(())
    }
}
