#![feature(box_syntax, question_mark)]

#[macro_use] extern crate clap;
extern crate regex;
extern crate sha1;
extern crate walkdir;

mod command;
mod dedup;
mod entry;
mod file;
mod hex;

use std::fs;
use std::path::Path;
use command::{Command, CommandOptions};
use entry::SortableDirEntry;
use file::FileIter;
use hex::Hex;

fn main() {
    match command::read() {
        Ok(Command::List(paths, options)) => list(paths, options),
        Ok(Command::Clean(paths, options)) => clean(paths, options),
        Err(e) => println!("{}", e),
    }
}

fn list<I, P>(paths: I, options: CommandOptions)
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>
{
    for (hash, group) in group_files(paths, &options) {
        println!("#{}", hash.hex());
        for item in group {
            println!(" {}", item.path().to_str().unwrap());
        }
    }
}

fn clean<I, P>(paths: I, options: CommandOptions)
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>    
{
    for (_, group) in group_files(paths, &options) {
        for file in group.iter().skip(1) {
            if options.verbose() {
                println!("removing: {}", file.path().display());
            }
            fs::remove_file(file.path()).ok();
        }
    }
}

fn group_files<I, P>(paths: I, options: &CommandOptions) -> Vec<(Vec<u8>, Vec<SortableDirEntry>)>
    where P: AsRef<Path>,
          I: IntoIterator<Item = P>
{
    let files = paths.into_iter()
        .flat_map(FileIter::new)
        .filter_map(|entry| if options.filter(entry.path()) {
            match entry.path().metadata().map(|data| data.len()) {
                Ok(len) => Some((len, SortableDirEntry::new(entry))),
                Err(_) => None,
            }
        } else {
            None
        });
        
    let files_by_size = dedup::group_by_size(files);
    dedup::group_by_hash(
        files_by_size.into_iter().flat_map(|group| dedup_group_by_path(group).into_iter()),
        options.verbose(),
    )
}

fn dedup_group_by_path(mut group: Vec<SortableDirEntry>) -> Vec<SortableDirEntry> {
    group.sort();
    group.dedup();
    group
}