use super::{Directory, Entry};
use crate::config::Config;
use std::{
    collections::{hash_map, HashMap},
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

// Note: we could consider a priority queue for dropping unused directories.
// Depth could be checked, where the root and any directories up to a depth
// of 2 won't be dropped, as they will likely be repopulated.

#[derive(Debug, Clone)]
pub struct Cache(HashMap<PathBuf, Directory>);

impl Cache {
    pub fn new() -> Self {
        Self(HashMap::<PathBuf, Directory>::new())
    }

    pub fn populate_to_root(&mut self, path: &Path, config: &Config) -> io::Result<()> {
        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            match self.as_mut().entry(curr.to_path_buf()) {
                hash_map::Entry::Occupied(mut entry) => {
                    let dir = entry.get_mut();
                    dir.reload(config)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = find_index_in(&dir.inner, ancestor) {
                            dir.index = Some(i);
                        }
                    }
                    prev = Some(curr);
                }
                hash_map::Entry::Vacant(entry) => {
                    let mut dir = Directory::new(curr.to_path_buf().clone(), config)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = find_index_in(&dir.inner, ancestor) {
                            dir.index = Some(i);
                        }
                    }
                    entry.insert(dir);
                    prev = Some(curr);
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, k: &Path) -> Option<&Directory> {
        self.as_ref().get(k)
    }

    pub fn set(&mut self, key: PathBuf, value: Directory) -> Option<Directory> {
        self.as_mut().insert(key, value)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn as_ref(&self) -> &HashMap<PathBuf, Directory> {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut HashMap<PathBuf, Directory> {
        &mut self.0
    }
}

impl Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.as_ref()
            .iter()
            .for_each(|(_, d)| buf += format!("{}\n", d).as_str());
        write!(f, "{}", buf)
    }
}

fn find_index_in(entries: &[Entry], path: &Path) -> Option<usize> {
    entries.iter().enumerate().find_map(|(i, dir)| {
        if dir.path.as_path() == path {
            Some(i)
        } else {
            None
        }
    })
}
