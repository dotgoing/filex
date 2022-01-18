use std::path::PathBuf;
pub struct FileInfo {
    pub entry: PathBuf,
    pub elapsed: u64,
    pub len: u64,
    pub acc_len: u64,
    pub reverse_acc_len: u64,
}
