use std::error::Error;
use clap::{builder, Command, ArgAction, Arg, ValueEnum, value_parser};
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Link => builder::PossibleValue::new("l"),
            Self::Dir => builder::PossibleValue::new("d"),
            Self::File => builder::PossibleValue::new("f"),
        })
    }
}


#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("Find in rust")
        .author("locnguyenvu")
        .version("0.0.1")
        .arg(
            Arg::new("paths")
            .action(ArgAction::Append)
            .value_name("PATH")
            .default_value(".")
            .help("Search paths")
        )
        .arg(
            Arg::new("names")
            .short('n')
            .long("name")
            .help("Name")
            .action(ArgAction::Append)
        )
        .arg(
            Arg::new("entry_types")
            .short('t')
            .long("type")
            .value_parser(value_parser!(EntryType))
            .help("Entry type")
            .action(ArgAction::Append)
        )
        .get_matches();

    let paths: Vec<String> = cmd.get_many::<String>("paths").unwrap().map(|e| e.to_string()).collect();

    let matched_names = cmd.get_many::<String>("names");
    let mut names: Vec<Regex> = vec![];
    if !matched_names.is_none() {
        for name in matched_names.unwrap() {
            match Regex::new(name) {
                Err(_) => {
                    std::panic!("Invalid --name \"{}\"", name);
                }
                Ok(r) => names.push(r)
            }
        }
    }

    let matched_entry_types = cmd.get_many::<EntryType>("entry_types");
    let mut entry_types: Vec<EntryType> = vec![];
    if !matched_entry_types.is_none() {
        entry_types = matched_entry_types.unwrap().map(|e| e.to_owned()).collect();
    }
    Ok(Config{ paths, names, entry_types })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &walkdir::DirEntry| {
        config.entry_types.is_empty()
        || config.entry_types.iter().any(|entry_type| {
            match entry_type {
                EntryType::Link => entry.file_type().is_symlink(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file()
            }})
    };
    let name_filter = |entry: &walkdir::DirEntry| {
        config.names.is_empty()
        || config.names.iter().any(|re| re.is_match(&entry.file_name().to_string_lossy(),))
    };
    for path in config.paths {
        let entries = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
