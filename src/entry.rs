use std::cmp::Ordering;
use std::io::Result;
use std::path::Path;
use walkdir::DirEntry;

pub trait Entry {
    fn path(&self) -> &Path;
    
    fn length(&self) -> Result<u64> {
        self.path().metadata().map(|meta| meta.len())
    }
}

impl Entry for DirEntry {
    fn path(&self) -> &Path {
        self.path()
    }
}

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

impl Entry for SortableDirEntry {
    fn path(&self) -> &Path {
        self.path()
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