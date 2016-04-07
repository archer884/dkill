use std::path::{Path, PathBuf};
use clap::ArgMatches;
use regex::Regex;

pub struct CommandOptions {
    include: Option<Regex>,
    exclude: Option<Regex>,
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
            match pattern {
                &None => return default,
                &Some(ref pattern) => pattern.is_match(path), 
            }
        }
        
        match_pattern(&path, &self.include, true) && !match_pattern(&path, &self.exclude, false)
    }
}

pub enum Command {
    List(Vec<PathBuf>, CommandOptions),
    Clean(Vec<PathBuf>, CommandOptions),
    Invalid(String),
}

pub fn read() -> Command {
    let matches = get_matches();
    match get_matches().subcommand() {
        ("list", Some(matches)) => list_command(matches),
        ("clean", Some(matches)) => clean_command(matches),
        _ => Command::Invalid(matches.usage().to_owned())
    }
}

fn list_command<'a>(matches: &'a ArgMatches<'a>) -> Command {
    Command::List(
        read_paths(matches),
        read_options(matches),
    )
}

fn clean_command<'a>(matches: &'a ArgMatches<'a>) -> Command {
    if matches.is_present("force") {
        Command::Clean(
            read_paths(matches),
            read_options(matches),
        )
    } else {
        Command::List(
            read_paths(matches),
            read_options(matches),
        )
    }
}

fn read_paths<'a>(matches: &'a ArgMatches<'a>) -> Vec<PathBuf> {
    matches.values_of("path").unwrap().map(|path| PathBuf::from(path)).collect() 
}

fn read_options<'a>(matches: &'a ArgMatches<'a>) -> CommandOptions {
    CommandOptions {
        include: matches.value_of("include").and_then(|pattern| Regex::new(pattern).ok()),
        exclude: matches.value_of("exclude").and_then(|pattern| Regex::new(pattern).ok()),        
    }
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
            (@arg include: -i --include ... "Regex pattern for files to be included")
            (@arg exclude: -e --exclude ... "Regex pattern for files to be excluded")
            (@arg path: ... +required "Path(s) to be listed")
        )
        (@subcommand clean =>
            (about: "Clean duplicate files. This command does nothing unless the --force parameter is supplied.")
            (version: "0.1.0")
            (author: "J/A <archer884@gmail.com>")
            (@arg force: -f --force "Force program to delete duplicates")
            (@arg include: -i --include ... "Regex pattern for files to be included")
            (@arg exclude: -e --exclude ... "Regex pattern for files to be excluded")
            (@arg path: ... +required "Path(s) to be cleaned")
        )
    ).get_matches()
}