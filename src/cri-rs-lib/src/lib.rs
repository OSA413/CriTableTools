use std::path::{Path, PathBuf};

pub mod aax {
    pub mod archive_mode;
    pub mod archive;
    pub mod entry_flag;
    pub mod entry;
}
pub mod cpk {
    pub mod archive;
    pub mod mode;
    pub mod entry;
}
pub mod csb {
    pub mod archive;   
}

pub fn find_cpk(path_to_search: &str) -> Option<PathBuf> {
    let path = &format!("{}.cpk", &path_to_search);
    let path = Path::new(path);
    if path.exists() {
        return Some(path.to_path_buf());
    }
    
    let path = &format!("{}.CPK", &path_to_search);
    let path = Path::new(path);
    if path.exists() {
        return Some(path.to_path_buf());
    }

    return None;
}