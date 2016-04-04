#![feature(box_syntax, question_mark)]

extern crate sha1;
extern crate walkdir;

mod command;
mod files;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use command::Command;
use files::{FileHash, FileIter};
use walkdir::DirEntry;

fn main() {
    match command::read() {
        Command::List(ref path) => list_dir(path),
        Command::Clean(ref path) => clean_dir(path),
        Command::Invalid => println!("please provide path"),
    }
}

fn list_dir<P: AsRef<Path>>(path: P) {
    for (hash, group) in group_files(path) {
        let hash: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        println!("Hash: {}", hash);
        
        for item in group {
            println!("\t{:?}", item);
        }
    }
}

fn clean_dir<P: AsRef<Path>>(path: P) {
    for (_, group) in group_files(path) {
        for file in group.iter().skip(1) {
            println!("Removing: {}", file.path().display());
            fs::remove_file(file.path()).ok();
        }
    }
}

fn group_files<P: AsRef<Path>>(path: P) -> Vec<(Vec<u8>, Vec<DirEntry>)> {
    let grouped_files = FileIter::new(path)
        .filter_map(|file| {
            println!("Processing: {}", file.path().display());
            FileHash::from_entry(file).ok()
        })
        .map(|filehash| (filehash.hash, filehash.entry))
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