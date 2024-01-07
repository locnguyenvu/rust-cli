use std::error::Error;
use clap::{Arg, ArgAction, Command, value_parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader};


type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => {
                eprintln!("Failed to open {}: {}", filename, err);
            },
            Ok(file) => {
                let mut skipblank = 0;
                for (idx, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", (idx + 1), line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                            skipblank = skipblank + 1;
                        } else {
                            println!("{:>6}\t{}", (idx + 1 - skipblank), line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .author("loc nguyen vu")
        .arg(Arg::new("number")
             .long("number")
             .short('n')
             .action(ArgAction::SetTrue)
             .value_parser(value_parser!(bool))
             .help("Number lines"))
        .arg(Arg::new("number_nonblank")
             .long("number-nonblank")
             .short('b')
             .action(ArgAction::SetTrue)
             .value_parser(value_parser!(bool))
             .conflicts_with("number")
             .help("Number nonblank lines"))
        .arg(Arg::new("file")
            .value_name("FILE")
            .value_parser(value_parser!(String))
            .default_value("-")
            .action(ArgAction::Append)
            .help("path to file")
    ).get_matches();

    let files: Vec<_> = matches.get_many::<String>("file").unwrap_or_default().map(|x| x.to_owned()).collect();
    Ok(Config {
        files,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}
