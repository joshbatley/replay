use crate::config::{ConfigFile, CURRENT_CMD_ID};
use clap::{Parser, Subcommand};
use rand::Rng;

const BASE_CONFIG_PATH: &str = "./config.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: Option<Vec<String>>,

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

    #[clap(skip)]
    pub command_friendly: String,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Run {
        #[clap(value_parser)]
        id: Option<String>,
    },
    Save {
        #[clap(value_parser)]
        cmd: Option<String>,

        #[clap(value_parser)]
        id: Option<String>,
    },
}

#[derive(Default, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Exec,
    ExecAndUpdate,
    Update,
}

impl Commands {
    pub fn new() -> Commands {
        let mut parsed = Commands::parse();
        let mut command_type = match &parsed.command {
            Some(Subcommands::Run { .. }) => CommandType::Exec,
            Some(Subcommands::Save { .. }) => CommandType::Update,
            None => CommandType::Exec,
        };
        let is_run = parsed.run.clone().is_some();
        if is_run {
            command_type = CommandType::ExecAndUpdate;
            parsed.command_friendly = parsed.run.clone().unwrap().join(" ");
        }
        parsed.command_type = command_type;
        return parsed;
    }

    pub fn get_command(&self, config: &impl ConfigFile) -> (String, String) {
        match &self.command {
            Some(Subcommands::Run { id }) => {
                let key = id.as_ref().unwrap().as_str();
                return (config.load_command(key), key.to_string());
            }
            Some(Subcommands::Save { cmd, id }) => {
                let rnd = rand::thread_rng().gen_range(0..100).to_string();
                let id = id.clone().unwrap_or(rnd);
                return (cmd.clone().unwrap(), id);
            }
            _ => (),
        };
        if self.run.is_none() {
            (
                config.load_command(CURRENT_CMD_ID),
                CURRENT_CMD_ID.to_string(),
            )
        } else {
            (self.command_friendly.clone(), CURRENT_CMD_ID.to_string())
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

        fn update_command(&mut self, _: &String, _: String) {}
    }

    #[test]
    fn get_last_command_gives_passed_command() {
        let cmd = vec![
            "echo".to_string(),
            "loaded".to_string(),
            "from".to_string(),
            "args".to_string(),
        ];
        let mock_config = ConfigTest {};
        let parse_command = Commands {
            run: Some(cmd.clone()),
            config: "".to_owned(),
            show_output: false,
            is_new_command: false,
            command: None,
            command_type: CommandType::Exec,
            command_friendly: cmd.join(" "),
        };
        let (command, _) = parse_command.get_command(&mock_config);

        assert_eq!(command, parse_command.command_friendly);
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
            command_type: CommandType::Exec,
            command_friendly: String::new(),
        };
        let (command, _) = parse_command.get_command(&mock_config);

        assert_eq!(command, TEST_SCRIPT);
    }
}
