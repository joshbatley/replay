use mockall_double::double;
use toml_edit::Document;

#[double]
use crate::file_api::FileApi;

pub trait ConfigFile {
    fn load_last_command(&self) -> &str;
    fn update_command(&mut self, cmd: &str);
}

pub struct Config {
    path: String,
    doc: Document,
}

const KEY: &str = "current";

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
    fn load_last_command(&self) -> &str {
        self.doc.get(KEY).unwrap().as_str().unwrap()
    }

    fn update_command(&mut self, cmd: &str) {
        self.doc[KEY] = toml_edit::value(cmd);
        self.update_file();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file_api::MockFileApi;

    #[test]
    fn update_command_updates_file() {
        let mut config = Config::new(&String::from(""));
        let ctx = MockFileApi::save_file_context();
        ctx.expect()
            .returning(move |_, file| assert_eq!(file, String::from("echo new script")));

        config.update_command(&String::from("echo new script"));
    }
}
