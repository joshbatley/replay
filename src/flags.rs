use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    // #[clap(value_parser)]
    // pub run: String,

    // #[clap(short, long, value_parser)]
    // pub update: String,

    // #[clap(short, long, value_parser)]
    // pub save: String,

    // #[clap(short, long, value_parser)]
    // pub delete: String,

    // #[clap(short, long, value_parser)]
    // pub edit: String,
    #[clap(short, long, value_parser)]
    pub list: bool,

    #[clap(short = 'o', long, value_parser)]
    pub show_output: bool,

    #[clap(short, long, value_parser)]
    pub verbose: bool,
}
