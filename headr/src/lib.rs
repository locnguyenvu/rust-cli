use clap::{Arg, ArgAction, Command, value_parser};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


#[warn(dead_code)]
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
    quite: bool,
}


pub fn parse_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .author("loc nguyen vu")
        .version("0.0.1")
        .arg(Arg::new("files")
             .action(ArgAction::Append)
             .required(true)
             .value_name("FILE")
             .value_parser(value_parser!(String))
             .help("Path to file")
        ).arg(Arg::new("bytes")
             .long("bytes")
             .short('c')
             .value_parser(value_parser!(u8))
             .help("Number of bytes")
        ).arg(Arg::new("lines")
             .long("lines")
             .short('n')
             .default_value("10")
             .value_parser(value_parser!(u8))
             .conflicts_with("bytes")
             .help("Number of lines")
        ).arg(Arg::new("quite")
             .long("quite")
             .alias("silent")
             .short('q')
             .value_parser(value_parser!(bool))
             .action(ArgAction::SetTrue)
        ).get_matches();
    let lines = *matches.get_one::<u8>("lines").unwrap();
    let bytes = matches.get_one::<u8>("bytes");
    let files: Vec<_> = matches.get_many::<String>("files").unwrap_or_default().map(|x| x.to_owned()).collect();
    let quite = matches.get_flag("quite");
    Ok(Config {
        files,
        lines: lines.into(),
        bytes: if bytes.is_none() { Option::None } else { Some(bytes.unwrap().to_owned().into()) },
        quite,
    })
}


pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match File::open(&filename) {
            Err(e) => {
                eprintln!("Failed to open file {}: {}", filename, e);
            },
            Ok(file) => {
                if !config.quite && config.files.len() > 1 {
                    println!("\n==> {} <==", filename);
                }
                if !config.bytes.is_none() {
                    let mut handle = file.take(config.bytes.unwrap() as u64);
                    let mut buffer = vec![0; config.bytes.unwrap()];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let lines = BufReader::new(file).lines();
                    for line in lines.take(config.lines) {
                        println!("{}", line?);
                    }
                }
            }
        }
    }
    Ok(())
}


// fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
//     Ok(Box::new(BufReader::new(File::open(filename).unwrap())))
// }
