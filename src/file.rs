use std::{fs, string};
use toml_edit::Document;

use crate::command::Cmd;

const FILE_LOCATION: &str = "./config.toml";
const CURRENT_CMD_ID: &str = "current";

// type Id = String;
// #[derive(Deserialize, Serialize)]
// #[serde(rename_all = "kebab-case")]
// pub struct Config {
//     current: Id,
//     scripts: HashMap<Id, Cmd>,
// }

pub struct File {
    raw_file: String,
    config: Document,
}
impl File {
    pub fn new() -> Self {
        let file = File::open_file();
        File {
            raw_file: file.clone(),
            config: file.parse::<Document>().unwrap(),
        }
    }
    fn open_file() -> String {
        fs::read_to_string(FILE_LOCATION).unwrap()
    }

    fn create_file() {}

    fn file_exist() -> bool {
        false
    }

    pub fn get_current_cmd(&self) -> Cmd {
        let current_id = self.config.get(CURRENT_CMD_ID).unwrap();
        self.get_cmd(current_id.as_str().unwrap())
    }

    pub fn save_file(&self) {
        let config_str = self.config.to_string();
        println!("{}", config_str);
        // fs::write(FILE_LOCATION, self.config.output()).unwrap()
    }

    pub fn get_cmd(&self, key: &str) -> Cmd {
        let all_scripts = self.config.get("scripts").unwrap();
        let cmd = Cmd::from_toml(all_scripts.get(key).unwrap());
        cmd.run_cmd();
        cmd
    }

    pub fn save_cmd(&mut self, key: &str, cmd: &Cmd) {
        let all_scripts = self.config.get_mut("scripts").unwrap();
        if all_scripts.get(key).is_some() {
            let script = all_scripts
                .get_mut(key)
                .unwrap()
                .as_table_like_mut()
                .unwrap();
            script.insert("script", toml_edit::value(cmd.script.to_string()));
            script.insert("added", toml_edit::value(cmd.added.to_string()));
            script.insert("timeout", toml_edit::value(cmd.timeout.to_string()));
        } else {
            all_scripts
                .as_table_mut()
                .unwrap()
                .insert(key, cmd.to_toml().to_owned());
        }
    }

    // pub fn show_all_scripts(self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn save_cmd_works() {
        let cmd = Cmd {
            script: "ls -lsa".to_string(),
            added: "2022-07-04T16:05:32.032Z".to_string(),
            timeout: 0,
            last_runs_successful: None,
            last_runs_output: None,
        };
        let mut file = File::new();
        file.save_cmd("new-script", &cmd);
        assert!(file.config.to_string().contains("new-script"));
    }

    #[test]
    fn save_cmd_updates() {
        let cmd = Cmd {
            script: "ls -lsa".to_string(),
            added: "2022-07-04T16:05:32.032Z".to_string(),
            timeout: 0,
            last_runs_successful: None,
            last_runs_output: None,
        };

        let mut file = File::new();
        file.save_cmd("example", &cmd);
        assert!(file.config.to_string().contains("ls -lsa"));
    }
}
