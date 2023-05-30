use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
    path::Path,
};

pub trait Writer {
    fn new(dest_folder: String) -> Self;
    fn write(&self, text: String, file_name: String) -> Result<(), Error>;
}

pub(crate) struct FileWriter {
    dest_folder: String,
}

impl Writer for FileWriter {
    fn new(dest_folder: String) -> Self {
        Self { dest_folder }
    }

    fn write(&self, text: String, file_name: String) -> Result<(), Error> {
        let mut file_path = self.dest_folder.to_owned();
        let file_n = file_name.to_owned();
        file_path.push_str(&file_n);
        let mut f: File;

        if Path::new(&file_path).exists() {
            f = OpenOptions::new()
                .create_new(false)
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            f.write(text.as_bytes())?;
        } else {
            f = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(file_path)
                .unwrap();
            f.write(text.as_bytes())?;
        }
        Ok(())
    }
}
