use std::{
    fmt, fs,
    io::{self, BufRead, BufReader},
    path::{self, PathBuf},
};

use super::{
    metadata::{self, FileType},
    Metadata,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub label: String,
    pub path: path::PathBuf,
    pub flagged: bool,
    pub metadata: Metadata,
}

impl Entry {
    pub fn from(dir_entry: &fs::DirEntry, show_icons: bool) -> io::Result<Self> {
        let name = dir_entry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .to_string();

        let path = dir_entry.path();
        let metadata = Metadata::from(&path)?;
        let flagged = false;
        let label = if show_icons {
            icon_label(&name, &path, &metadata)?
        } else {
            name.clone()
        };

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

    pub fn preview(&self, lines: usize) -> Result<String, Box<dyn std::error::Error>> {
        match self.metadata.file_type {
            metadata::FileType::File => {
                let raw = self.read_n_lines(lines)?;
                Ok(String::from_utf8(raw)?)
            }
            metadata::FileType::Directory(size) => Ok(format!("dir size: {}", size)),
        }
    }

    fn read_n_lines(&self, lines: usize) -> io::Result<Vec<u8>> {
        let file = fs::File::open(&self.path)?;
        let line_reader = BufReader::new(file).lines();

        let mut buf: Vec<u8> = Vec::new();
        for (i, line) in line_reader.enumerate() {
            if i > lines {
                break;
            }
            buf.append(&mut (line.unwrap() + "\n").as_bytes().to_vec());
        }
        Ok(buf)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

fn icon_label(name: &String, path: &PathBuf, md: &Metadata) -> io::Result<String> {
    use super::icon::*;

    let icon = match md.file_type {
        FileType::Directory(_) => DIR_NODE_EXACT_MATCHES
            .get(name.as_str())
            .cloned()
            .unwrap_or(DEFAULT_DIR),
        _ => FILE_NODE_EXACT_MATCHES
            .get(name.as_str())
            .cloned()
            .unwrap_or(match path.extension() {
                Some(s) => FILE_NODE_EXTENSIONS
                    .get(match s.to_str() {
                        Some(s) => s,
                        None => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Failed converting OsStr to str",
                            ))
                        }
                    })
                    .unwrap_or(&DEFAULT_FILE),
                None => DEFAULT_FILE,
            }),
    };
    Ok(format!("{} {}", icon, name))
}
