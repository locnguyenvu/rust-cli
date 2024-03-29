use clap::{Command, Arg, ArgAction};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
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
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(filename) => Box::new(File::create(filename)?),
        _ => Box::new(std::io::stdout())
    };
    let mut print = |count: usize, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?
            } else {
                write!(out_file, "{}", text)?
            }
        }
        Ok(())
    };
    let mut line = String::new();
    let mut previous = String::new();
    let mut count = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }
    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

