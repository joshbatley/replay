use crate::{config::Config, flags::Commands};
use clap::Parser;
use log::LevelFilter;

mod command;
mod config;
mod file_utils;
mod flags;
mod test;

fn main() {
    let cmd = Commands::parse();
    let config = Config::new(cmd.config);
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
