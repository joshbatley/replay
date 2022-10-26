use crate::config::{ConfigFile, CURRENT_CMD_ID};
use clap::{Args, Parser, Subcommand};

const BASE_CONFIG_PATH: &str = "./config.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: Option<String>,

    #[command(subcommand)]
    pub command: Option<Subcommands>,

    // Globals
    #[clap(short, long, value_parser, default_value_t = String::from(BASE_CONFIG_PATH), global = true)]
    pub config: String,

    #[clap(short, long, value_parser, default_value_t = false, global = true)]
    pub show_output: bool,

    /// Internal state
    #[clap(skip)]
    pub is_new_command: bool,

    #[clap(skip)]
    pub command_type: CommandType,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Run(Flags),
}

#[derive(Args)]
pub struct Flags {
    #[clap(value_parser)]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Eq)]
pub enum CommandType {
    Exec,
    Update,
    #[default]
    None,
}

impl Commands {
    pub fn new() -> Commands {
        let mut parsed = Commands::parse();
        let command_type = match &parsed.command {
            Some(Subcommands::Run(_)) => CommandType::Exec,
            None => CommandType::None,
        };
        if parsed.run.is_some() && command_type != CommandType::Exec {
            parsed.is_new_command = true;
        }
        parsed.command_type = command_type;
        return parsed;
    }

    pub fn get_command(&self, config: &impl ConfigFile) -> (String, &str) {
        match &self.command {
            Some(Subcommands::Run(cmd)) => {
                let key = cmd.id.as_ref().unwrap().as_str();
                return (config.load_command(key), key);
            }
            _ => (),
        };
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
            is_new_command: false,
            command: None,
            command_type: CommandType::None,
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
            is_new_command: false,
            command: None,
            command_type: CommandType::None,
        };
        let (command, _) = parse_command.get_command(&mock_config);

        assert_eq!(command, TEST_SCRIPT);
    }
}
