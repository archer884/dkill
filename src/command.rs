use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use regex::Regex;

pub struct CommandOptions {
    include: Option<Regex>,
    exclude: Option<Regex>,
    verbose: bool,
}

impl CommandOptions {
    pub fn filter<P: AsRef<Path>>(&self, path: P) -> bool {
        // If this path is unrepresentable, we are not examining it.
        let path = match path.as_ref().to_str() {
            None => return false,
            Some(path) => path,
        };
        
        // I have no idea why I can't just map this. Any time I try to use map for this,
        // it complains that I'm not allowed to move borrowed things. 
        fn match_pattern(path: &str, pattern: &Option<Regex>, default: bool) -> bool {
            match *pattern {
                None => default,
                Some(ref pattern) => pattern.is_match(path), 
            }
        }
        
        match_pattern(&path, &self.include, true) && !match_pattern(&path, &self.exclude, false)
    }
    
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

pub enum Command {
    List(Vec<PathBuf>, CommandOptions),
    Clean(Vec<PathBuf>, CommandOptions),
}

#[derive(Debug)]
pub enum CommandError {
    BadIncludePattern(Box<Error>),
    BadExcludePattern(Box<Error>),
    BadPathHierarchy,
    InvalidCommand(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::BadIncludePattern(ref e) => write!(f, "Error! Failed to compile include pattern:\n\t{}", e),
            CommandError::BadExcludePattern(ref e) => write!(f, "Error! Failed to compile exclude pattern:\n\t{}", e),
            CommandError::BadPathHierarchy => write!(f, "Error! A path provided for inspection overlaps with another path provided for inspection"),
            CommandError::InvalidCommand(ref usage) => write!(f, "{}", usage),
        }
    }
} 

pub fn read() -> Result<Command, CommandError> {
    let matches = get_matches();
    match get_matches().subcommand() {
        ("list", Some(matches)) => list_command(matches),
        ("clean", Some(matches)) => clean_command(matches),
        _ => Err(CommandError::InvalidCommand(matches.usage().to_owned())),
    }
}

fn list_command<'a>(matches: &'a ArgMatches<'a>) -> Result<Command, CommandError> {
    Ok(Command::List(
        read_paths(matches)?,
        read_options(matches)?,
    ))
}

fn clean_command<'a>(matches: &'a ArgMatches<'a>) -> Result<Command, CommandError> {
    if matches.is_present("force") {
        Ok(Command::Clean(
            read_paths(matches)?,
            read_options(matches)?,
        ))
    } else {
        Ok(Command::List(
            read_paths(matches)?,
            read_options(matches)?,
        ))
    }
}

fn read_paths<'a>(matches: &'a ArgMatches<'a>) -> Result<Vec<PathBuf>, CommandError> {
    let paths: Vec<_> = matches.values_of("path").unwrap().map(PathBuf::from).collect();
    if is_non_overlapping(&paths) {
        Ok(paths)
    } else {
        Err(CommandError::BadPathHierarchy)
    }
}

fn read_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CommandOptions, CommandError> {
    Ok(CommandOptions {
        include: match matches.value_of("include").map(|pattern| Regex::new(pattern)) {
            Some(Err(e)) => return Err(CommandError::BadIncludePattern(box e)),
            Some(Ok(regex)) => Some(regex),
            None => None,
        },
        exclude: match matches.value_of("exclude").map(|pattern| Regex::new(pattern)) {
            Some(Err(e)) => return Err(CommandError::BadExcludePattern(box e)),
            Some(Ok(regex)) => Some(regex),
            None => None,
        },
        verbose: matches.is_present("verbose"),
    })
}

fn is_non_overlapping(paths: &[PathBuf]) -> bool {
    paths.iter().enumerate().all(|(idx_a, path_a)| 
        paths.iter().enumerate().all(|(idx_b, path_b)|
            idx_a == idx_b || !path_b.starts_with(path_a)
    ))
}

fn get_matches<'a>() -> ArgMatches<'a> {
    clap_app!(myapp => 
        (version: "0.1.0")
        (author: "J/A <archer884@gmail.com>")
        (about: "A simple file deduplication program.")
        (@subcommand list =>
            (about: "List duplicate files.")
            (version: "0.1.0")
            (author: "J/A <archer884@gmail.com>")
            (@arg verbose: -v --verbose "Verbose mode")
            (@arg include: -i --include +takes_value ... "Regex pattern for files to be included")
            (@arg exclude: -e --exclude +takes_value ... "Regex pattern for files to be excluded")
            (@arg path: ... +required "Path(s) to be listed")
        )
        (@subcommand clean =>
            (about: "Clean duplicate files. This command does nothing unless the --force parameter is supplied.")
            (version: "0.1.0")
            (author: "J/A <archer884@gmail.com>")
            (@arg force: -f --force "Force program to delete duplicates")
            (@arg verbose: -v --verbose "Verbose mode")
            (@arg include: -i --include +takes_value ... "Regex pattern for files to be included")
            (@arg exclude: -e --exclude +takes_value ... "Regex pattern for files to be excluded")
            (@arg path: ... +required "Path(s) to be cleaned")
        )
    ).get_matches()
}