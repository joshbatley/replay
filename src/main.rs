use clap::Parser;
use std::{
    fs,
    process::{Command, Output},
};

const BASE_CONFIG_PATH: &str = "./config.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: Option<String>,
    #[clap(short, long, value_parser, default_value_t = String::from(BASE_CONFIG_PATH))]
    pub config: String,
}

fn main() {
    let parse_commands = Commands::parse();
    let cmd = get_command(parse_commands);

    match run_cmd(&cmd) {
        Ok(res) => println!("{}", String::from_utf8(res.stdout).unwrap()),
        Err(_) => println!("Command could not be ran"),
    };
}

fn get_command(parse_commands: Commands) -> String {
    if parse_commands.run.is_none() {
        load_last_cmd(&parse_commands.config)
    } else {
        parse_commands.run.unwrap().to_owned()
    }
}

fn load_last_cmd(path: &String) -> String {
    if fs::metadata(path).is_ok() {
        fs::read_to_string(path).unwrap()
    } else {
        panic!("asdas")
    }
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
    use std::{fs, io::Write};

    use super::*;

    const TEST_FILE: &str = "./test.toml";
    const TEST_SCRIPT: &str = "echo loaded from file";

    pub struct TestFile;

    fn setup() -> TestFile {
        let mut file = fs::File::create(TEST_FILE).unwrap();
        file.write(TEST_SCRIPT.as_bytes()).unwrap();
        TestFile {}
    }
    impl Drop for TestFile {
        fn drop(&mut self) {
            fs::remove_file(TEST_FILE).unwrap();
        }
    }

    #[test]
    fn get_last_command_gives_passed_command() {
        let cmd = String::from("echo loaded from args");
        let parse_command = Commands {
            run: Some(cmd.to_owned()),
            config: TEST_FILE.to_owned(),
        };
        let command = get_command(parse_command);

        assert_eq!(command, cmd)
    }

    #[test]
    fn get_last_command_reads_from_file_if_no_commands_supplied() {
        let _s = setup();
        let parse_command = Commands {
            run: None,
            config: TEST_FILE.to_owned(),
        };
        let command = get_command(parse_command);

        assert_eq!(command, TEST_SCRIPT)
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
}
