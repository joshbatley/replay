use crate::config;
use std::fs;

const BASE_CONFIG_PATH: &str = "./config2.toml";
const BASE_CONFIG_FILE: &str = "current = \"\"\n\n[scripts]";

pub fn open_file(path: Option<String>) -> (String, String) {
    let path = if path.is_some() {
        let path = path.unwrap();
        if !is_valid(&path) {
            panic!("asdas")
        }
        path
    } else {
        if !file_exist(BASE_CONFIG_PATH) {
            create_file()
        }
        BASE_CONFIG_PATH.to_string()
    };

    (path.to_string(), fs::read_to_string(path).unwrap())
}

pub fn is_valid(path: &str) -> bool {
    file_exist(path) && config::is_valid_config(path)
}

pub fn create_file() {
    save_file(BASE_CONFIG_PATH.to_string(), BASE_CONFIG_FILE.to_string());
}

pub fn file_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn save_file(file: String, path: String) {
    fs::write(path, file).unwrap()
}
