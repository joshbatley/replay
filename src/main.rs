use commands::Commands;
use config::{Config, ConfigFile};
use std::process::{Command, Output};

mod commands;
mod config;
mod file_api;

fn main() {
    let parse_commands = Commands::new();
    let mut config = Config::new(&parse_commands.config);
    let (cmd, key) = parse_commands.get_command(&config);

    if parse_commands.is_new_command {
        config.update_command(&cmd, key);
    }

    match run_cmd(&cmd) {
        Ok(res) => print_output(&parse_commands.show_output, res),
        Err(_) => println!("Command could not be ran"),
    };
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

#[cfg(test)]
mod test {
    use super::*;

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
}
