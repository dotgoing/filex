use anyhow::bail;
use anyhow::Result;
use std::fs;
use std::fs::DirEntry;

mod error;
mod info;
use error::MyError;
pub use info::FileInfo;

pub fn parse_file(entry: DirEntry) -> Result<FileInfo> {
    let metadata = fs::metadata(&entry.path())?;
    if !metadata.is_file() {
        bail!(MyError::NotFile);
    }
    let elapsed = metadata.created()?.elapsed()?.as_secs();
    let info = FileInfo {
        entry: entry.path(),
        elapsed,
        len: metadata.len(),
    };
    Ok(info)
}
