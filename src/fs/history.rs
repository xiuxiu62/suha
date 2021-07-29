use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};
use std::{default, io};

use super::{Directory, Entry};
use crate::options::DisplayOptions;

pub struct History {
    pub inner: HashMap<PathBuf, Directory>,
}

impl History {
    pub fn new() -> Self {
        let inner: HashMap<PathBuf, Directory> = HashMap::new();
        Self { inner }
    }

    pub fn populate_to_root(&mut self, path: &Path, options: &DisplayOptions) -> io::Result<()> {
        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            match self.inner.entry(curr.to_path_buf()) {
                hash_map::Entry::Occupied(mut entry) => {
                    let dir = entry.get_mut();
                    dir.reload(options)?;
                    if let Some(ancestor) = prev.as_ref() {
                        if let Some(i) = find_index_in(&dir.inner, ancestor) {
                            dir.index = Some(i);
                        }
                    }
                    prev = Some(curr);
                }
                hash_map::Entry::Vacant(entry) => {
                    let mut dir = Directory::new(curr.to_path_buf().clone(), options)?;
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
