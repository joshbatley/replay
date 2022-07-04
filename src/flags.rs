use clap::{clap_derive::ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(value_parser)]
    pub run: String,

    #[clap(short, long, value_parser)]
    pub update: String,

    #[clap(short, long, value_parser)]
    pub save: String,

    #[clap(short, long, value_parser)]
    pub delete: String,

    #[clap(short, long, value_parser)]
    pub edit: String,

    #[clap(short, long, value_parser)]
    pub list: bool,

    #[clap(arg_enum, short, long, value_parser, default_value=LogLevels::Info )]
    pub verbose: LogLevels,
}

#[derive(Copy, Clone, ArgEnum)]
pub enum LogLevels {
    Debug,
    Info,
}
