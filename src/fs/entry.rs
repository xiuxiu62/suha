use std::{fmt, fs, io, path};

use super::Metadata;

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub label: String,
    pub path: path::PathBuf,
    pub flagged: bool,
    pub metadata: Metadata,
}

impl Entry {
    pub fn from(direntry: &fs::DirEntry, show_icons: bool) -> io::Result<Self> {
        let name = direntry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .to_string();
        let label = name.clone();
        let path = direntry.path();
        let flagged = false;
        let metadata = Metadata::from(&path)?;

        Ok(Self {
            name,
            label,
            path,
            flagged,
            metadata,
        })
    }

    pub fn get_extension(&self) -> &str {
        let name = self.name.as_str();
        if let Some(i) = name.rfind('.') {
            &name[i..]
        } else {
            ""
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
