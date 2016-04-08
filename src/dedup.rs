use std::collections::HashMap;
use file::FileHash;
use walkdir::DirEntry;

pub fn group_by_size<I>(paths: I) -> Vec<Vec<DirEntry>>
    where I: Iterator<Item = (u64, DirEntry)>
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

pub fn group_by_hash<I>(paths: I, verbose: bool) -> Vec<(Vec<u8>, Vec<DirEntry>)>
    where I: Iterator<Item = DirEntry>
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