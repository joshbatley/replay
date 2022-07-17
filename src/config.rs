use crate::{command::Cmd, file_utils};
use toml_edit::Document;

const CURRENT_CMD_ID: &str = "current";

#[derive(Clone)]
pub struct Config {
    raw_file: String,
    doc: Document,
    path: String,
}

pub fn is_valid_config(path: &str) -> bool {
    false
}

impl Config {
    pub fn new(path: Option<String>) -> Self {
        let (path, file) = file_utils::open_file(path);
        Config {
            raw_file: file.clone(),
            doc: file.parse::<Document>().unwrap(),
            path,
        }
    }
    fn get_current_cmd(&self) -> Cmd {
        let current_id = self.doc.get(CURRENT_CMD_ID).unwrap();
        self.get_cmd(current_id.as_str().unwrap())
    }

    pub fn save_file(&self) {
        file_utils::save_file(self.doc.to_string(), self.path.to_string())
    }

    pub fn get_cmd(&self, key: &str) -> Cmd {
        let all_scripts = self.doc.get("scripts").unwrap();
        let cmd = Cmd::from_toml(all_scripts.get(key).unwrap());
        cmd.run_cmd();
        cmd
    }

    pub fn save_cmd(&mut self, key: &str, cmd: &Cmd) {
        let all_scripts = self.doc.get_mut("scripts").unwrap();
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
        self.save_file();
    }
}

#[cfg(test)]
mod test {
    use crate::test::{setup, TEST_FILE};

    use super::*;

    #[test]
    fn save_cmd_works() {
        let s = setup();
        let cmd = Cmd {
            script: "ls -lsa".to_string(),
            added: "2022-07-04T16:05:32.032Z".to_string(),
            timeout: 0,
            last_runs_successful: None,
            last_runs_output: None,
        };
        let mut file = Config::new(Some(TEST_FILE.to_string()));
        file.save_cmd("new-script", &cmd);
        assert!(file.doc.to_string().contains("new-script"));
    }

    #[test]
    fn save_cmd_updates() {
        let s = setup();
        let cmd = Cmd {
            script: "ls -lsa".to_string(),
            added: "2022-07-04T16:05:32.032Z".to_string(),
            timeout: 0,
            last_runs_successful: None,
            last_runs_output: None,
        };

        let mut file = Config::new(Some(TEST_FILE.to_string()));
        file.save_cmd("example", &cmd);
        assert!(file
            .doc
            .to_string()
            .contains("example = { script = \"ls -lsa\""));
    }
}
