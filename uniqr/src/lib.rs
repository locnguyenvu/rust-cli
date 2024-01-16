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

#[derive(Debug)]
struct UniqObj {
    text: String,
    count: usize,
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
    // dbg!(config);
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut output: Vec<UniqObj> = Vec::new();
    let mut current_line = String::new();
    let mut current_count = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if current_line.is_empty() {
            current_line = line.clone();
        }
        if current_line != line {
            output.push(UniqObj { 
                text: current_line.clone(), 
                count: current_count }
            );
            current_line = line.clone();
            current_count = 1;
        } else {
            current_count += 1;
        }
        line.clear();
    }
    output.push(UniqObj { 
        text: current_line, 
        count: current_count }
    );
    let mut output_writer = output_writer(&config)?;
    for elem in output.iter() {
        let outfmt: String;
        if config.count {
            outfmt = format!("{:>8} {}", elem.count, elem.text);
        } else {
            outfmt = format!("{}", elem.text);
        }
        let _ = output_writer.write_all(outfmt.as_bytes());
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn output_writer(config: &Config) -> MyResult<Box<dyn Write>> {
    if config.out_file.is_none() {
        Ok(Box::new(io::stdout()))
    }
    else {
        let outfile = config.out_file.clone().unwrap();
        Ok(Box::new(File::create(outfile)?))
    }
}
