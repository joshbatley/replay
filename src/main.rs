use crate::flags::Commands;
use clap::{Parser};
use log::LevelFilter;

mod flags;
mod files;

fn main() {
    let cmd = Commands::parse();
    setup_logger(cmd.verbose, cmd.show_output);
    // flags(&cmd);
    files::create_file();
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

