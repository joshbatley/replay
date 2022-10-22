use mockall_double::double;

#[double]
use crate::file_api::FileApi;

pub trait ConfigFile {
    fn load_last_command(&self, path: &String) -> String;
    fn update_command(&self, path: &String, cmd: &str);
}

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        return Config {};
    }
}

impl ConfigFile for Config {
    fn load_last_command(&self, path: &String) -> String {
        FileApi::read_file(path)
    }

    fn update_command(&self, path: &String, cmd: &str) {
        FileApi::save_file(path, cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file_api::MockFileApi;

    pub const TEST_FILE: &str = "./test.toml";

    #[test]
    fn update_command_updates_file() {
        let config = Config::new();
        let ctx = MockFileApi::save_file_context();
        ctx.expect()
            .returning(move |_, file| assert_eq!(file, String::from("echo new script")));

        config.update_command(&TEST_FILE.to_string(), &String::from("echo new script"));
    }
}
