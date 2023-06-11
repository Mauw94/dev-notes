use std::io::Error;

use tauri::api::path::home_dir;

const APP_FOLDER: &str = "dev-notes";
const NOTES_FOLDER: &str = "notes";
const LOGIN_NAME: &str = "shimy";
const LOGIN_PASS: &str = "123";

pub struct Config {
    pub app_folder: String,
    pub files_folder: String,
    pub test_file_name: String,
    pub test_file_path: String,
    pub login_name: String,
    pub login_pass: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            app_folder: Self::app_dir().unwrap(),
            files_folder: Self::files_dir(),
            test_file_name: String::from("test_file.txt"),
            test_file_path: Self::test_file_path(),
            login_name: LOGIN_NAME.to_string(),
            login_pass: LOGIN_PASS.to_string()
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

    pub(crate) fn test_file_path() -> String {
        Self::files_dir() + "test_file.txt"
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
