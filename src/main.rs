#![feature(box_syntax, question_mark)]

#[macro_use] extern crate clap;
extern crate regex;
extern crate sha1;
extern crate walkdir;

mod command;
mod file;
mod hex;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use command::{Command, CommandOptions};
use file::{FileHash, FileIter};
use hex::Hex;
use walkdir::DirEntry;

fn main() {
    match command::read() {
        Command::List(paths, options) => list(paths, options),
        Command::Clean(paths, options) => clean(paths, options),
        Command::Invalid(usage) => println!("{}", usage),
    }
}

fn list<I, P>(paths: I, options: CommandOptions)
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>
{
    for (hash, group) in group_files(paths, &options) {
        println!("Hash: {}", hash.hex());
        
        for item in group {
            println!("\t{:?}", item);
        }
    }
}

fn clean<I, P>(paths: I, options: CommandOptions)
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>    
{
    for (_, group) in group_files(paths, &options) {
        for file in group.iter().skip(1) {
            println!("Removing: {}", file.path().display());
            fs::remove_file(file.path()).ok();
        }
    }
}

fn group_files<I, P>(paths: I, options: &CommandOptions) -> Vec<(Vec<u8>, Vec<DirEntry>)>
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>
{
    let grouped_files = paths.into_iter()
        .flat_map(|path| FileIter::new(path))
        .filter(|entry| options.filter(entry.path()))
        .filter_map(|entry| FileHash::from_entry(entry).map(|filehash| (filehash.hash, filehash.entry)).ok())
        .fold(HashMap::new(), |mut map, (hash, entry)| {
            map.entry(hash).or_insert(Vec::new()).push(entry);
            map
        });
        
    grouped_files.into_iter()
        .filter(|&(_, ref group)| group.len() > 1)
        .map(|(key, mut group)| {
            group.sort_by_key(|entry| entry.path().to_str().map(|s| s.len()).unwrap_or(0));
            (key, group)
        })
        .collect()
}