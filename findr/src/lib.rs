use crate::EntryType::*;
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

impl EntryType {
    pub fn possible_values() -> impl Iterator<Item = builder::PossibleValue> {
        Self::value_variants()
            .iter()
            .filter_map(ValueEnum::to_possible_value)
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
            .required(true)
            .default_value(".")
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
            .help("Entry type [possible values: f, d, l]")
        )
        .get_matches();

    let paths: Vec<String> = cmd.get_many::<String>("paths").unwrap().map(|e| e.to_string()).collect();
    let names: Vec<Regex> = cmd.get_many::<String>("names").unwrap().map(|e| Regex::new(e).unwrap()).collect();
    let entry_types: Vec<EntryType> = cmd.get_many::<EntryType>("entry_types").unwrap().map(|e| e.to_owned()).collect();
    Ok(Config{ paths, names, entry_types })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
