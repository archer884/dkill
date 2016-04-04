use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Result};
use sha1::Sha1;
use walkdir;
use walkdir::DirEntry;

pub struct FileIter {
    iter: walkdir::Iter
}

impl FileIter {
    pub fn new<P: AsRef<Path>>(path: P) -> FileIter {
        FileIter {
            iter: walkdir::WalkDir::new(path.as_ref()).into_iter()
        }
    }
}

impl Iterator for FileIter {
    type Item = DirEntry;
    
    fn next(&mut self) -> Option<DirEntry> {
        loop {
            match self.iter.next() {
                Some(Ok(path)) => if path.file_type().is_file() {
                    return Some(path)
                },
                None => return None,
                _ => continue,
            }
        }
    }
}

#[derive(Debug)]
pub struct FileHash {
    pub entry: DirEntry,
    pub hash: Vec<u8>,
}

impl FileHash {
    pub fn from_entry(entry: DirEntry) -> Result<FileHash> {
        Ok(FileHash {
            hash: hash_file(&entry.path())?,
            entry: entry,
        })
    }
}

impl fmt::Display for FileHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash: String = self.hash.iter().map(|b| format!("{:02x}", b)).collect();
        write!(f, "{{{}: {}}}", self.entry.path().display(), hash)
    }
}

fn hash_file(path: &Path) -> Result<Vec<u8>> {
    match File::open(path) {
        Err(e) => Err(e),
        Ok(mut file) => {
            let mut buffer = box [0u8; 8388608];
            let mut hash = Sha1::new();
            
            loop {
                let bytes_read = file.read(&mut *buffer)?;
                if bytes_read > 0 {
                    hash.update(&buffer[0..bytes_read]);
                }
                else {
                    break;
                }
            }
            
            Ok(hash.digest())
        }
    }
}