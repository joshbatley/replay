use mockall_double::double;
use toml_edit::Document;

#[double]
use crate::file_api::FileApi;

pub const CURRENT_CMD_ID: &str = "current";

pub trait ConfigFile {
    fn load_command(&self, key: &str) -> String;
    fn update_command(&mut self, cmd: &String, key: String);
}

pub struct Config {
    path: String,
    doc: Document,
}

impl Config {
    pub fn new(path: &String) -> Self {
        let file = FileApi::read_file(path);
        return Config {
            path: path.to_owned(),
            doc: file.parse::<Document>().unwrap(),
        };
    }

    fn update_file(&self) {
        FileApi::save_file(&self.path, &self.doc.to_string())
    }
}

impl ConfigFile for Config {
    fn load_command(&self, key: &str) -> String {
        // Add better error
        self.doc.get(key).unwrap().as_str().unwrap().to_string()
    }

    fn update_command(&mut self, cmd: &String, key: String) {
        self.doc[&key] = toml_edit::value(cmd);
        self.update_file();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file_api::MockFileApi;

    #[test]
    fn update_command_updates_file() {
        let mut config = Config {
            path: String::from(""),
            doc: Document::new(),
        };
        let ctx = MockFileApi::save_file_context();
        ctx.expect().returning(move |_, file| {
            assert_eq!(file, String::from("current = \"echo new script\"\n"))
        });

        config.update_command(&String::from("echo new script"), CURRENT_CMD_ID.to_string());
    }
}
