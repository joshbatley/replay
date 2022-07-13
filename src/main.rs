use crate::flags::Commands;
use clap::Parser;
use file::File;
use log::LevelFilter;

mod command;
mod file;
mod flags;

fn main() {
    // if (setup) {
    //     // create file
    // }

    // if (update_aviable) {
    //     // check if update
    // }

    let config = File::new();
    let cmd = Commands::parse();
    setup_logger(cmd.verbose, cmd.show_output);
    // flags(&cmd);

    config.get_cmd("example");
}

fn setup_logger(is_verbose: bool, show_output: bool) {
    let level: LevelFilter = if is_verbose {
        LevelFilter::Debug
    } else if show_output {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };
    env_logger::Builder::new().filter_level(level).init();
}

// fn setup() {}

// fn update() {}
