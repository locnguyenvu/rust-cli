use clap::{Command, Arg, ArgAction};
use std::error::Error;
use std::ops::Range;
use std::str::FromStr;
use regex::Regex;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use csv::{ReaderBuilder,StringRecord, WriterBuilder};


type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("cutr")
        .author("locnguyenvu")
        .version("0.0.1")
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .action(ArgAction::Append)
            .default_value("-")
        )
        .arg(
            Arg::new("delimiter")
            .short('d')
            .long("delim")
            .action(ArgAction::Set)
            .default_value("\t")
            .help("Field delimiter")
        )
        .arg(
            Arg::new("bytes")
            .short('b')
            .long("bytes")
            .value_name("BYTES")
            .help("Select bytes")
            .action(ArgAction::Append)
            .value_delimiter(',')
            .conflicts_with_all(["fields", "chars"])
        )
        .arg(
            Arg::new("chars")
            .short('c')
            .long("chars")
            .value_name("CHARS")
            .help("Select characters")
            .action(ArgAction::Append)
            .value_delimiter(',')
            .conflicts_with_all(["bytes", "fields"])
        )
        .arg(
            Arg::new("fields")
            .short('f')
            .long("fields")
            .value_name("FIELDS")
            .help("Select fields")
            .value_delimiter(',')
            .conflicts_with_all(["chars", "bytes"])
        ).get_matches();

    let extract: Extract = if cmd.get_many::<String>("fields").is_some() {
        let mut parse_fields = cmd.get_many::<String>("fields").unwrap().map(parse_index);
        let mut pl: PositionList = vec![];
        for _i in 0..parse_fields.len() {
            match parse_fields.next().unwrap() {
                Ok(range) => { pl.push(range) },
                Err(err) => { return Err(err); }
            }
        }
        Extract::Fields(pl)
    } else if cmd.get_many::<String>("chars").is_some() {
        let mut parse_fields = cmd.get_many::<String>("chars").unwrap().map(parse_index);
        let mut pl: PositionList = vec![];
        for _i in 0..parse_fields.len() {
            match parse_fields.next().unwrap() {
                Ok(range) => { pl.push(range) },
                Err(err) => { return Err(err); }
            }
        }
        Extract::Chars(pl)
    } else {
        let mut parse_fields = cmd.get_many::<String>("bytes").unwrap().map(parse_index);
        let mut pl: PositionList = vec![];
        for _i in 0..parse_fields.len() {
            match parse_fields.next().unwrap() {
                Ok(range) => { pl.push(range) },
                Err(err) => { return Err(err); }
            }
        }
        Extract::Bytes(pl)
    };


    let delimiter = cmd.get_one::<String>("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        return Err(From::from(format!("--delim \"{}\" must be a single byte", delimiter)));
    }

    Ok(Config{
        files: cmd.get_many::<String>("files").unwrap().map(|e| e.to_string()).collect(),
        delimiter: *delim_bytes.first().unwrap(),
        extract,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => match &config.extract {
                Extract::Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(config.delimiter)
                        .has_headers(false)
                        .from_reader(file);
                    let mut wtr = WriterBuilder::new()
                        .delimiter(config.delimiter)
                        .from_writer(io::stdout());
                    for record in reader.records() {
                        let record = record?;
                        wtr.write_record(extract_fields(
                            &record,
                            field_pos
                        ))?;
                    }
                }
                Extract::Bytes(byte_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }
                Extract::Chars(char_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_index(txt: &String) -> Result<Range<usize>, Box<dyn Error>> {
    let range_regx = Regex::new(r"^(?<start>\d+)(\-)*(?<end>\d+)*$").unwrap();
    if !range_regx.is_match(txt) {
        return Err(From::from(format!("Invalid range \"{}\"", txt)));
    }
    let caps = range_regx.captures(txt).unwrap();
    let start: usize = usize::from_str(&caps["start"]).unwrap() - 1;
    let end: usize = caps.name("end").map_or(start+1, |v| usize::from_str(v.into()).unwrap() + 1).to_owned();
    Ok(Range{ start, end })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    let mut selected: Vec<char> = vec![];

    for range in char_pos.iter().cloned() {
        selected.extend(range.filter_map(|i| chars.get(i)))
    }
    selected.iter().collect()
}

fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i).copied()))
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields<'a>(
    records: &'a StringRecord,
    field_pos: &[Range<usize>]
) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| records.get(i)))
        .collect()
}
