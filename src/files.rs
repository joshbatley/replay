use std::fs;

use log::warn;
use serde::{Deserialize, Serialize};
use toml::value::Table;

#[derive(Deserialize, Serialize, Debug)]
pub struct FileFormat {
  current: String,
  scripts: Table,
  #[serde(skip)]
  scipts_vec: Vec<String>
}

impl FileFormat {
  fn new(file: &str) -> FileFormat {
    let mut parsed = toml::from_str::<FileFormat>(file).unwrap();
    parsed.scipts_vec = parsed.scripts.keys().into_iter().map(|f| f.to_string()).collect();
    parsed
  }
}

pub fn create_file() {

}

pub fn save_file() {
  warn!("here");
  let file = fs::read_to_string("./config.toml").expect("Unable to read file");
  let parsed = FileFormat::new(file.as_str());
  let values = parsed.scripts.get(&parsed.scipts_vec[1]);
  warn!("---------{:?}", values);
}

