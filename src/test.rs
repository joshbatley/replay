use std::{fs, io::Write};

pub const TEST_FILE: &str = "./test.toml";

pub struct TestFile;

pub fn setup() -> TestFile {
    let mut file = fs::File::create(TEST_FILE).unwrap();
    file.write(b"current = 'example'\n\n[config]\nversion = 0.1\n\n[scripts]\nexample = { script = 'curl https://jsonplaceholder.typicode.com/todos/1', added = '2022-07-04T16:05:32.032Z', timeout = 0 }\nexample2 = { script = 'curl https://localhost:5002', added = '2022-07-04T16:05:32.032Z', timeout = 0 }").unwrap();
    TestFile {}
}

impl Drop for TestFile {
    fn drop(&mut self) {
        fs::remove_file(TEST_FILE).unwrap();
    }
}
