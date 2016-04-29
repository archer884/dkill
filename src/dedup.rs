use std::collections::HashMap;
use entry::Entry;
use file::FileHash;

pub fn group_by_size<E, I>(paths: I) -> Vec<Vec<E>>
    where E: Entry,
          I: Iterator<Item = (u64, E)>
{
    let groups = paths.fold(HashMap::new(), |mut map, (len, entry)| {
        map.entry(len).or_insert_with(Vec::new).push(entry);
        map
    });
    
    groups.into_iter()
        .map(|(_, group)| group)
        .filter(|group| group.len() > 1)
        .collect()
}

pub fn group_by_hash<E, I>(paths: I, verbose: bool) -> Vec<(Vec<u8>, Vec<E>)>
    where E: Entry + Ord,
          I: Iterator<Item = E>
{
    let groups = paths.filter_map(|entry| {
            if verbose {
                println!("processing: {}", entry.path().display());
            }
            FileHash::from_entry(entry).map(|filehash| (filehash.hash, filehash.entry)).ok()
        })
        .fold(HashMap::new(), |mut map, (hash, entry)| {
            map.entry(hash).or_insert_with(Vec::new).push(entry);
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