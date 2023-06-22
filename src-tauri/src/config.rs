use std::io::Error;

use tauri::api::path::home_dir;

const APP_FOLDER: &str = "dev-notes";
const NOTES_FOLDER: &str = "notes";
const LOGIN_NAME: &str = "shimy";
const LOGIN_PASS: &str = "123";
const NOTE_META_DATA_FOLDER: &str = "meta";
const TEST_FILE_PATH: &str = "test_file.txt";

pub struct Config {
    pub app_folder: String,
    pub files_folder: String,
    pub note_meta_data_folder: String,
    pub test_file_name: String,
    pub test_file_path: String,
    pub login_name: String,
    pub login_pass: String,
}

// TODO create note_config with required and readable properties?
impl Config {
    pub fn default() -> Self {
        Self {
            app_folder: Self::app_dir().unwrap(),
            files_folder: Self::files_dir(),
            note_meta_data_folder: Self::meta_files_dir(),
            test_file_name: TEST_FILE_PATH.to_string(),
            test_file_path: Self::test_file_path(),
            login_name: LOGIN_NAME.to_string(),
            login_pass: LOGIN_PASS.to_string(),
        }
    }

    fn app_dir() -> Result<String, Error> {
        let dir = home_dir().unwrap().into_os_string().into_string().unwrap();
        Ok(dir + "\\" + APP_FOLDER + "\\")
    }

    fn files_dir() -> String {
        let base_dir = Self::app_dir().unwrap();
        base_dir + NOTES_FOLDER + "\\"
    }

    fn meta_files_dir() -> String {
        let base_dir = Self::app_dir().unwrap();
        base_dir + NOTE_META_DATA_FOLDER + "\\"
    }

    pub fn test_file_path() -> String {
        Self::files_dir() + TEST_FILE_PATH
    }

    pub fn test_meta_file_path() -> String {
        Self::meta_files_dir() + TEST_FILE_PATH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_has_correct_app_folder() {
        let cfg = Config::default();
        let home_folder = home_dir().unwrap().into_os_string().into_string().unwrap();
        assert_eq!(
            cfg.app_folder,
            home_folder.clone() + "\\" + APP_FOLDER + "\\"
        );
    }

    #[test]
    fn config_has_correct_notes_folder() {
        let cfg = Config::default();
        let home_folder = home_dir().unwrap().into_os_string().into_string().unwrap();
        assert_eq!(
            cfg.files_folder,
            home_folder.clone() + "\\" + APP_FOLDER + "\\" + NOTES_FOLDER + "\\"
        );
    }
}
