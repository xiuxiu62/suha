use std::slice::{Iter, IterMut};
use std::{fs, io, path};

use super::{Entry, Metadata};

#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub show_hidden: bool,
    pub show_icons: bool,
}

impl DisplayOptions {
    pub fn new(show_hidden: bool, show_icons: bool) -> Self {
        Self {
            show_hidden,
            show_icons,
        }
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub inner: Vec<Entry>,
    pub index: Option<usize>,
    pub metadata: Metadata,
    path: path::PathBuf,
}

impl Directory {
    pub fn new(path: path::PathBuf, options: &DisplayOptions) -> io::Result<Self> {
        let inner = read_dir_list(path.as_path(), options)?;
        let index = if inner.is_empty() { None } else { Some(0) };
        let metadata = Metadata::from(&path)?;

        Ok(Self {
            inner,
            index,
            metadata,
            path,
        })
    }

    pub fn reload(&mut self, options: &DisplayOptions) -> io::Result<()> {
        let inner = read_dir_list(&self.path, options)?;
        let inner_len = inner.len();
        let index: Option<usize> = if inner_len == 0 {
            None
        } else {
            match self.index {
                Some(i) if i >= inner_len => Some(inner_len - 1),
                Some(i) => {
                    let entry = &self.inner[i];
                    inner
                        .iter()
                        .enumerate()
                        .find(|(_, e)| e.name == entry.name)
                        .map(|(i, _)| i)
                        .or(Some(i))
                }
                None => Some(0),
            }
        };

        let metadata = Metadata::from(&self.path)?;
        self.metadata = metadata;
        self.inner = inner;
        self.index = index;

        Ok(())
    }

    pub fn iter(&self) -> Iter<Entry> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Entry> {
        self.inner.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn modified(&self) -> bool {
        let metadata = fs::symlink_metadata(&self.path);
        match metadata {
            Ok(m) => match m.modified() {
                Ok(s) => s > self.metadata.modified,
                _ => false,
            },
            _ => false,
        }
    }
}

fn read_dir_list(path: &path::Path, options: &DisplayOptions) -> io::Result<Vec<Entry>> {
    let filter_fn = if options.show_hidden {
        show_hidden
    } else {
        no_hidden
    };
    let results: Vec<Entry> = fs::read_dir(path)?
        .filter(filter_fn)
        .filter_map(|res| Entry::from(&res.ok()?, options.show_icons).ok())
        .collect();
    Ok(results)
}

const fn show_hidden(_: &Result<fs::DirEntry, io::Error>) -> bool {
    true
}

fn no_hidden(result: &Result<fs::DirEntry, io::Error>) -> bool {
    match result {
        Err(_) => true,
        Ok(entry) => {
            let file_name = entry.file_name();
            let lossy_string = file_name.as_os_str().to_string_lossy();
            !lossy_string.starts_with('.')
        }
    }
}
