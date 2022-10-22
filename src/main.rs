use clap::Parser;
use mockall_double::double;
use std::process::{Command, Output};

#[double]
use file_api::FileApi;

mod file_api;

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

fn main() {
    let parse_commands = Commands::parse();
    let (cmd, is_new_cmd) = get_command(&parse_commands);

    if is_new_cmd {
        update_command(&parse_commands.config, &cmd);
    }

    match run_cmd(&cmd) {
        Ok(res) => print_output(&parse_commands.show_output, res),
        Err(_) => println!("Command could not be ran"),
    };
}

fn get_command(parse_commands: &Commands) -> (String, bool) {
    if parse_commands.run.is_none() {
        (load_last_cmd(&parse_commands.config), false)
    } else {
        let command = parse_commands.run.as_ref().unwrap();
        (command.to_string(), true)
    }
}

fn run_cmd(cmd: &String) -> Result<Output, std::io::Error> {
    if let [first, tail @ ..] = &cmd.split_whitespace().collect::<Vec<&str>>()[..] {
        Command::new(&first).args(tail).output()
    } else {
        panic!("Need to set up correct error")
    }
}

fn print_output(show_output: &bool, response: Output) {
    if !show_output {
        return;
    }
    let output = String::from_utf8(response.stdout).unwrap();
    println!("{}", output)
}

fn load_last_cmd(path: &String) -> String {
    FileApi::read_file(path)
}

fn update_command(path: &String, cmd: &str) {
    FileApi::save_file(path, cmd)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file_api::MockFileApi;
    pub const TEST_FILE: &str = "./test.toml";
    pub const TEST_SCRIPT: &str = "echo loaded from file";

    #[test]
    fn get_last_command_gives_passed_command() {
        let cmd = String::from("echo loaded from args");
        let parse_command = Commands {
            run: Some(cmd.to_owned()),
            config: "".to_owned(),
            show_output: false,
        };
        let (command, new_command) = get_command(&parse_command);

        assert_eq!(command, cmd);
        assert_eq!(new_command, true);
    }

    #[test]
    fn get_last_command_reads_from_file_if_no_commands_supplied() {
        let ctx = MockFileApi::read_file_context();
        ctx.expect().return_const(TEST_SCRIPT);

        let parse_command = Commands {
            run: None,
            config: TEST_FILE.to_owned(),
            show_output: false,
        };
        let (command, new_command) = get_command(&parse_command);

        assert_eq!(command, TEST_SCRIPT);
        assert_eq!(new_command, false);
    }

    #[test]
    fn run_cmd_should_return_response_of_valid_cmd() {
        let test_command = String::from("echo hello world");
        let result = run_cmd(&test_command).unwrap();
        let result_str = String::from_utf8(result.stdout).unwrap();

        assert_eq!("hello world\n", result_str);
    }

    #[test]
    fn run_cmd_should_return_error_if_command_failed() {
        let test_command = String::from("ehco hello world");
        let result = run_cmd(&test_command);

        assert!(result.is_err());
    }

    #[test]
    fn update_command_updates_file() {
        let ctx = MockFileApi::save_file_context();
        ctx.expect()
            .returning(move |_, file| assert_eq!(file, String::from("echo new script")));

        update_command(&TEST_FILE.to_string(), &String::from("echo new script"));
    }
}
