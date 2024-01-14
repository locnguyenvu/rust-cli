use clap::{Arg, ArgAction, Command};
use clap::parser::ValueSource;
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    lines: usize,
    bytes: usize,
    chars: usize,
    words: usize,
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("Word count command in rust")
        .version("0.0.1")
        .author("loc nguyen vu")
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .default_value("-")
            .help("Input files")
            .action(ArgAction::Append)
        )
        .arg(
            Arg::new("lines")
            .short('l')
            .long("lines")
            .help("Show line count")
            .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("words")
            .short('w')
            .long("words")
            .help("Show word count")
            .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("bytes")
            .short('c')
            .long("bytes")
            .help("Show bytes count")
            .action(ArgAction::SetTrue)
        ).arg(
            Arg::new("chars")
            .short('m')
            .long("chars")
            .help("Show chars count")
            .action(ArgAction::SetTrue)
            .conflicts_with("bytes")
        ).get_matches();

    let files = cmd.get_many::<String>("files").unwrap().map(|e| e.to_owned()).collect();

    
    let mut lines = cmd.get_flag("lines");
    let mut words = cmd.get_flag("words");
    let mut bytes = cmd.get_flag("bytes");

    if ["lines", "words", "bytes"].iter().all(|k| cmd.value_source(k) == Some(ValueSource::DefaultValue)) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config{
        files,
        lines,
        words,
        bytes,
        chars: cmd.get_flag("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0usize;
    let mut total_words = 0usize;
    let mut total_chars = 0usize;
    let mut total_bytes = 0usize;
    for filename in &config.files {
        match open(filename) {
            Err(e) => {
                eprintln!("{}: {}", filename, e);
            }
            Ok(file) => {
                let info = count(file)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(info.lines, config.lines),
                    format_field(info.words, config.words),
                    format_field(info.bytes, config.bytes),
                    format_field(info.chars, config.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", &filename)
                    },
                );
                total_lines += info.lines;
                total_words += info.words;
                total_bytes += info.bytes;
                total_chars += info.chars;
            }
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{}{}",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
            " total".to_string()
        );
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

// fn count(mut bread: impl BufRead) -> MyResult<FileInfo> {
//     let file = &mut bread;
//     let mut lines: usize = 0;
//     let mut chars: usize = 0;
//     let mut words: usize = 0;
//     let mut bytes: usize = 0;
//     for line in file.lines() {
//         let line = &line?;
//         let line_bytes = line.clone().into_bytes().len();
//         words = words + line.split(' ').collect::<Vec<_>>().len();
//         chars = chars + line.chars().count();
//         lines = lines + 1;
//         bytes = bytes + line_bytes + 2;
//     }
// 
//     Ok(FileInfo{
//         lines,
//         bytes,
//         chars,
//         words,
//     })
// }


fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut lines: usize = 0;
    let mut chars: usize = 0;
    let mut words: usize = 0;
    let mut bytes: usize = 0;
    let mut line_str = String::new();
    
    loop {
        let read_bytes = file.read_line(&mut line_str)?;
        if read_bytes == 0 {
            break;
        }
        lines += 1;
        bytes += read_bytes;
        words += line_str.split_whitespace().count();
        chars += line_str.chars().count(); 
    }

    Ok(FileInfo{
        lines,
        bytes,
        chars,
        words,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let cursor = Cursor::new(text);
        let info = count(cursor);
        assert!(info.is_ok());
        let expected = FileInfo {
            lines: 1,
            words: 10,
            chars: 48,
            bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
