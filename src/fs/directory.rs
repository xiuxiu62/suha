use super::{Entry, Metadata};
use crate::{config::Config, ui::Component};

use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use std::{
    fmt::Display,
    fs, io,
    path::{self, Path},
    slice::{Iter, IterMut},
};

#[derive(Debug, Clone)]
pub struct Directory {
    pub path: path::PathBuf,
    pub inner: Vec<Entry>,
    pub index: Option<usize>,
    pub metadata: Metadata,
}

impl Directory {
    pub fn new(path: path::PathBuf, config: &Config) -> io::Result<Self> {
        let inner = read_dir_list(path.as_path(), config)?;
        let index = if inner.is_empty() { None } else { Some(0) };
        let metadata = Metadata::from(&path)?;

        Ok(Self {
            path,
            inner,
            index,
            metadata,
        })
    }

    pub fn reload(&mut self, config: &Config) -> io::Result<()> {
        let inner = read_dir_list(&self.path, config)?;
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

    pub fn parent(&self, config: &Config) -> io::Result<Option<Self>> {
        if let Some(path) = self.path.parent() {
            Ok(Some(Directory::new(path.to_path_buf(), config)?))
        } else {
            Ok(None)
        }
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

fn read_dir_list(path: &path::Path, config: &Config) -> io::Result<Vec<Entry>> {
    let results = fs::read_dir(path)?
        .filter(|res| {
            if config.show_hidden {
                true
            } else {
                match res {
                    Err(_) => true,
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        let lossy_string = file_name.as_os_str().to_string_lossy();
                        !lossy_string.starts_with('.')
                    }
                }
            }
        })
        .filter_map(|res| Entry::from(&res.ok()?, config.show_icons).ok())
        .collect();
    Ok(results)
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .inner
            .iter()
            .fold(String::new(), |acc, file| format!("{}\n{}", acc, file));

        write!(f, "{}", body)
    }
}

impl Component<Paragraph<'static>> for Directory {
    fn draw(&self) -> Paragraph<'static> {
        let title = format!("[ {} ]", self.path.to_string_lossy().as_ref());
        let body = self.to_string();

        Paragraph::new(body).block(Block::default().title(title))
    }
}
