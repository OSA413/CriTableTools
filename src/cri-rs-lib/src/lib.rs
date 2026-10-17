use std::{ffi::OsStr, path::{Path, PathBuf}};

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

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn find_related_file(path_to_search: &Path, extension: &OsStr) -> Option<PathBuf> {
    let path = &format!("{}.{}", &path_to_search.to_string_lossy(), extension.to_string_lossy());
    let path = Path::new(path);
    path.exists().then(|| path.to_path_buf())
}

pub fn find_related_cpk(csb_path: &Path) -> Option<PathBuf> {
    let base_directory = csb_path.parent().unwrap();
    let path_to_search = base_directory.join(csb_path.file_stem().unwrap());

    find_related_file(&path_to_search, OsStr::new("CPK"))
        .or_else(|| find_related_file(&path_to_search, OsStr::new("cpk")))
}