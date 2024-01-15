use clap::{Command, Arg, ArgAction};
use std::error::Error;


type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("uniqr")
        .version("0.1.0")
        .author("locnguyenvu <loc.nguyenvu@outlook.com>")
        .arg(
            Arg::new("in_file")
            .value_name("IN_FILE")
            .default_value("-")
            .action(ArgAction::Set)
            .help("Input file")
        )
        .arg(
            Arg::new("out_file")
            .action(ArgAction::Set)
            .value_name("OUT_FILE")
            .help("Output file")
        )
        .arg(
            Arg::new("count")
            .short('c')
            .long("count")
            .help("Show counts")
            .action(ArgAction::SetTrue)
        ).get_matches();

    Ok(Config {
        in_file: cmd.get_one::<String>("in_file").map(String::from).unwrap(),
        out_file: cmd.get_one::<String>("out_file").map(String::from),
        count: cmd.get_flag("count")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
