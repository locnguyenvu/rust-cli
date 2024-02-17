use clap::{Command, Arg, ArgAction};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;



type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<MyResult<String>>,
    count: bool,
    recursive: bool,
    invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("grepr")
        .author("locnguyenvu")
        .version("0.0.1")
        .arg(
            Arg::new("pattern")
            .value_name("PATTERN")
            .action(ArgAction::Set)
            .required(true)
        )
        .arg(
            Arg::new("files")
            .value_name("FILE")
            .action(ArgAction::Append)
            .default_value("-")
        )
        .arg(
            Arg::new("insensitive")
            .short('i')
            .long("insensitive")
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("recursive")
            .short('r')
            .long("recursive")
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("count")
            .short('c')
            .long("count")
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("invert_match")
            .short('v')
            .long("invert-match")
            .action(ArgAction::SetTrue)
        ).get_matches();

    let count = cmd.get_flag("count");
    let recursive = cmd.get_flag("recursive");
    let invert_match = cmd.get_flag("invert_match");
    let insensitive = cmd.get_flag("insensitive");
    let pattern_str = cmd.get_one::<String>("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern_str)
        .case_insensitive(insensitive)
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", pattern_str))?;
    let files: Vec<String> = cmd.get_many::<String>("files").unwrap().map(|e| e.to_owned()).collect();

    Ok(Config{
        files: find_files(&files, recursive),
        pattern,
        recursive,
        count,
        invert_match,
    })
}

pub fn run(config: Config) -> MyResult<()>{
    for filename in &config.files {
        match filename {
            Ok(filename) => match open(filename) {
                Err(e) => { eprintln!("Failed to open {}: {}", filename, e) }
                Ok(f) => {
                    let printmatch = |text: &String| {
                        if config.recursive || config.files.len() > 1 {
                            println!("{}:{}", filename, text);
                        } else {
                            println!("{}", text);
                        }
                    };
                    match find_lines(f, &config.pattern, config.invert_match) {
                        Ok(lines) => {
                            if config.count {
                                printmatch(&lines.len().to_string());
                            } else {
                                lines.iter().for_each(printmatch);
                            }
                        },
                        Err(e) => {
                            return Err(From::from(format!("{}", e)));
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => { Ok(Box::new(BufReader::new(std::io::stdin()))) }
        _ => { Ok(Box::new(BufReader::new(File::open(filename)?))) }
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut result: Vec<_> = vec![];

    for path in paths {
        match path.as_str() {
            "-" => result.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| !e.file_type().is_dir())
                            {
                                result.push(Ok(entry
                                    .path()
                                    .display()
                                    .to_string()));
                            }
                        } else {
                            result.push(Err(From::from(format!(
                                "{}: Is a directory",
                                path
                            ))));
                        }
                    } else if metadata.is_file() {
                        result.push(Ok(path.to_string()));
                    }
                }
                Err(e) => {
                    result.push(Err(From::from(format!("{}: {}", path, e))))
                }
            }
        }
    }
    result
}

fn find_lines<T: BufRead>(
    file: T,
    pattern: &Regex,
    invert_match: bool,
) -> MyResult<Vec<String>> {
    let mut result: Vec<String> = vec![];
    for line in file.lines() {
        let line = &line.unwrap();
        if pattern.is_match(line) {
            if !invert_match { 
                result.push(line.to_string());
            }
        } else {
            if invert_match {
                result.push(line.to_string());
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files =
            find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs: Is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        println!("{:?}", &files);
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
