// use crate::flags::Commands;
use clap::Parser;
// use flags::LogLevels;
use log::LevelFilter;

// mod flags;

// fn main() {
//     let cmd = Commands::parse();
//     setup_logger(&cmd.verbose);
//     // flags(&cmd);
// }

// fn setup_logger(is_verbose: &LogLevels) {
//     let level: LevelFilter = match is_verbose {
//         LogLevels::Debug => LevelFilter::Debug,
//         LogLevels::Info => LevelFilter::Info,
//     };
//     env_logger::Builder::new().filter_level(level).init();
// }

// fn flags(cmd: &ArgMatches) {
//     let run = cmd.get_one::<bool>(VERBOSE_KEY).unwrap();
//     let run2 = cmd.get_one::<bool>(LIST_KEY).unwrap();
//     warn!("{}{}", run, run2);
// }

use clap::ArgEnum;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[clap(
        short,
        long,
        value_parser(["always", "auto", "never"]),
        default_missing_value("always")
    )]
    mode: String,
}
// fn func(s: &str) -> Result<&str, String> {
//     match s {
//         "Fast" => Ok(),
//         "Slow" => Ok(Mode::Slow),
//         "" => Ok(Mode::Fast),
//         _ => Err("Select Fast or slow or none".to_string()),
//     }
// }

fn main() {
    let cli = Cli::parse();
    println!("{}",cli.mode)
    // match cli.mode {
    //     Mode::Fast => {
    //         println!("Hare");
    //     }
    //     Mode::Slow => {
    //         println!("Tortoise");
    //     }
    // }
}
