use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use file::FileHash;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct SortableDirEntry(DirEntry);

impl SortableDirEntry {
    pub fn new(entry: DirEntry) -> SortableDirEntry {
        SortableDirEntry(entry)
    }
    
    pub fn path(&self) -> &Path {
        self.0.path()
    }
}

impl PartialEq for SortableDirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path().eq(other.path())
    }
}

impl Eq for SortableDirEntry { }

impl PartialOrd for SortableDirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path().partial_cmp(other.path())
    }
}

impl Ord for SortableDirEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path().cmp(other.path())
    }
}

pub fn group_by_size<I>(paths: I) -> Vec<Vec<SortableDirEntry>>
    where I: Iterator<Item = (u64, SortableDirEntry)>
{
    let groups = paths.fold(HashMap::new(), |mut map, (len, entry)| {
        map.entry(len).or_insert(Vec::new()).push(entry);
        map
    });
    
    groups.into_iter()
        .map(|(_, group)| group)
        .filter(|group| group.len() > 1)
        .collect()
}

pub fn group_by_hash<I>(paths: I, verbose: bool) -> Vec<(Vec<u8>, Vec<SortableDirEntry>)>
    where I: Iterator<Item = SortableDirEntry>
{
    let groups = paths.filter_map(|entry| {
            if verbose {
                println!("processing: {}", entry.path().display());
            }
            FileHash::from_entry(entry).map(|filehash| (filehash.hash, filehash.entry)).ok()
        })
        .fold(HashMap::new(), |mut map, (hash, entry)| {
            map.entry(hash).or_insert(Vec::new()).push(entry);
            map
        });
        
    groups.into_iter()
        .filter(|&(_, ref group)| group.len() > 1)
        .map(|(key, mut group)| {
            group.sort_by_key(|entry| entry.path().to_str().unwrap().len());
            (key, group)
        })
        .collect()
}