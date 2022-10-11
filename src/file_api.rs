use mockall::automock;
use std::fs;

pub struct FileApi;

#[automock()]
impl FileApi {
    pub fn read_file(path: &String) -> String {
        if !FileApi::is_valid_path(path) {
            panic!("Config file not found")
        }

        fs::read_to_string(path).unwrap()
    }

    pub fn is_valid_path(path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    pub fn save_file(path: &String, contents: &str) {
        fs::write(path, contents).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const TEST_FILE: &str = "./test2.toml";
    pub const TEST_SCRIPT: &str = "echo loaded from file";

    #[test]
    fn file_utils() {
        // Create
        FileApi::save_file(&String::from(TEST_FILE), &String::from(TEST_SCRIPT));
        // Check valid
        let is_valid = FileApi::is_valid_path(TEST_FILE);
        assert!(is_valid);
        // read
        let contents = FileApi::read_file(&String::from(TEST_FILE));
        assert_eq!(contents, TEST_SCRIPT);
        // update
        let new_content = &String::from("update in test");
        FileApi::save_file(&String::from(TEST_FILE), new_content);

        let content = FileApi::read_file(&String::from(TEST_FILE));
        assert_eq!(content, new_content.to_owned());

        fs::remove_file(TEST_FILE).unwrap()
    }
}
