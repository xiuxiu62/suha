use std::collections::{hash_map, HashMap};
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

use super::{Directory, Entry};
use crate::context::Flags;

// Note: we could consider a priority queue for dropping unused directories.
// Depth could be checked, where the root and any directories up to a depth
// of 2 won't be dropped, as they will likely be repopulated.

#[derive(Debug, Clone)]
pub struct Cache {
    pub inner: HashMap<PathBuf, Directory>,
}

impl Cache {
    pub fn new() -> Self {
        let inner: HashMap<PathBuf, Directory> = HashMap::new();
        Self { inner }
    }

    pub fn populate_to_root(&mut self, path: &Path, flags: &Flags) -> io::Result<()> {
        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            match self.inner.entry(curr.to_path_buf()) {
                hash_map::Entry::Occupied(mut entry) => {
                    let dir = entry.get_mut();
                    dir.reload(flags)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = find_index_in(&dir.inner, ancestor) {
                            dir.index = Some(i);
                        }
                    }
                    prev = Some(curr);
                }
                hash_map::Entry::Vacant(entry) => {
                    let mut dir = Directory::new(curr.to_path_buf().clone(), flags)?;
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

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl Display for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.inner
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
