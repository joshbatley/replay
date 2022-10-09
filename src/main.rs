use clap::Parser;
use std::process::{Command, Output};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: String,
}

fn main() {
    let cmd = Commands::parse();
    match run_cmd(&cmd.run) {
        Ok(res) => println!("{}", String::from_utf8(res.stdout).unwrap()),
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
