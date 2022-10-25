use crate::config::{ConfigFile, CURRENT_CMD_ID};
use clap::Parser;

const BASE_CONFIG_PATH: &str = "./config.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: Option<String>,

    #[clap(short, long, value_parser, default_value_t = String::from(BASE_CONFIG_PATH))]
    pub config: String,

    #[clap(short, long, value_parser, default_value_t = false)]
    pub show_output: bool,

    #[clap(short, long, value_parser)]
    pub run_command: Option<String>,

    #[clap(skip)]
    pub is_new_command: bool,
}

impl Commands {
    pub fn new() -> Commands {
        let mut parsed = Commands::parse();
        if parsed.run.is_none() && parsed.run_command.is_none() {
            parsed.is_new_command = true;
        }
        return parsed;
    }

    pub fn get_command(&self, config: &impl ConfigFile) -> (String, &str) {
        if self.run_command.is_some() {
            let key = self.run_command.as_ref().unwrap().as_str();
            return (config.load_command(key), key);
        }

        if self.run.is_none() {
            (config.load_command(CURRENT_CMD_ID), CURRENT_CMD_ID)
        } else {
            let command = self.run.as_ref().unwrap();
            (command.to_owned(), CURRENT_CMD_ID)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_SCRIPT: &str = "echo loaded from file";

    struct ConfigTest {}
    impl ConfigFile for ConfigTest {
        fn load_command(&self, _: &str) -> String {
            TEST_SCRIPT.to_owned()
        }

        fn update_command(&mut self, _: &String, _: &str) {}
    }

    #[test]
    fn get_last_command_gives_passed_command() {
        let cmd = String::from("echo loaded from args");
        let mock_config = ConfigTest {};
        let parse_command = Commands {
            run: Some(cmd.to_owned()),
            config: "".to_owned(),
            show_output: false,
            run_command: None,
            is_new_command: false,
        };
        let (command, _) = parse_command.get_command(&mock_config);

        assert_eq!(command, cmd);
    }

    #[test]
    fn get_last_command_reads_from_file_if_no_commands_supplied() {
        let mock_config = ConfigTest {};
        let parse_command = Commands {
            run: None,
            config: "".to_owned(),
            show_output: false,
            run_command: None,
            is_new_command: false,
        };
        let (command, _) = parse_command.get_command(&mock_config);

        assert_eq!(command, TEST_SCRIPT);
    }
}
