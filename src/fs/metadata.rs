use std::{fs, io, path, time};

#[derive(Debug, Clone, Copy)]
enum FileType {
    File,
    Directory(usize),
}

#[derive(Debug, Clone)]
enum LinkType {
    Normal,
    Symlink(String),
}

#[derive(Debug, Clone, Copy)]
pub struct UnixData {
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    len: u64,
    pub modified: time::SystemTime,
    permissions: fs::Permissions,
    file_type: FileType,
    link_type: LinkType,
    #[cfg(unix)]
    pub unix_data: UnixData,
}

impl Metadata {
    pub fn from(path: &path::Path) -> io::Result<Self> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let symlink_metadata = fs::symlink_metadata(path)?;
        let metadata = fs::metadata(path)?;

        let len = metadata.len();
        let modified = metadata.modified()?;
        let permissions = metadata.permissions();
        let file_type = if metadata.is_dir() {
            let dir_size = fs::read_dir(path).map(|s| s.count())?;
            FileType::Directory(dir_size)
        } else {
            FileType::File
        };
        let link_type = match symlink_metadata.file_type().is_symlink() {
            true => {
                let mut link = "".to_string();

                if let Ok(path) = fs::read_link(path) {
                    if let Some(s) = path.to_str() {
                        link = s.to_string()
                    }
                }

                LinkType::Symlink(link)
            }
            false => LinkType::Normal,
        };

        #[cfg(unix)]
        let unix_data = UnixData {
            uid: symlink_metadata.uid(),
            gid: symlink_metadata.gid(),
            mode: symlink_metadata.mode(),
        };

        Ok(Self {
            len,
            modified,
            permissions,
            file_type,
            link_type,
            unix_data,
        })
    }
}
