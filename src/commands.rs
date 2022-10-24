use clap::Parser;

use crate::config::ConfigFile;

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
}

impl Commands {
    pub fn new() -> Commands {
        Commands::parse()
    }

    pub fn get_command(&self, config: &impl ConfigFile) -> (String, bool) {
        if self.run.is_none() {
            (config.load_last_command(), false)
        } else {
            let command = self.run.as_ref().unwrap();
            (command.to_owned(), true)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_SCRIPT: &str = "echo loaded from file";

    struct ConfigTest {}
    impl ConfigFile for ConfigTest {
        fn load_last_command(&self) -> String {
            TEST_SCRIPT.to_owned()
        }

        fn update_command(&self, _: &str) {}
    }

    #[test]
    fn get_last_command_gives_passed_command() {
        let cmd = String::from("echo loaded from args");
        let mock_config = ConfigTest {};
        let parse_command = Commands {
            run: Some(cmd.to_owned()),
            config: "".to_owned(),
            show_output: false,
        };
        let (command, new_command) = parse_command.get_command(&mock_config);

        assert_eq!(command, cmd);
        assert_eq!(new_command, true);
    }

    #[test]
    fn get_last_command_reads_from_file_if_no_commands_supplied() {
        let mock_config = ConfigTest {};
        let parse_command = Commands {
            run: None,
            config: "".to_owned(),
            show_output: false,
        };
        let (command, new_command) = parse_command.get_command(&mock_config);

        assert_eq!(command, TEST_SCRIPT);
        assert_eq!(new_command, false);
    }
}
