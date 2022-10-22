use mockall_double::double;

#[double]
use crate::file_api::FileApi;

pub trait ConfigFile {
    fn load_last_command(&self) -> String;
    fn update_command(&self, cmd: &str);
}

pub struct Config {
    path: String,
}

impl Config {
    pub fn new(path: &String) -> Self {
        return Config {
            path: path.to_owned(),
        };
    }
}

impl ConfigFile for Config {
    fn load_last_command(&self) -> String {
        FileApi::read_file(&self.path)
    }

    fn update_command(&self, cmd: &str) {
        FileApi::save_file(&self.path, cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file_api::MockFileApi;
    pub const TEST_FILE: &str = "./test.toml";

    #[test]
    fn update_command_updates_file() {
        let config = Config::new(&String::from(""));
        let ctx = MockFileApi::save_file_context();
        ctx.expect()
            .returning(move |_, file| assert_eq!(file, String::from("echo new script")));

        config.update_command(&String::from("echo new script"));
    }
}
