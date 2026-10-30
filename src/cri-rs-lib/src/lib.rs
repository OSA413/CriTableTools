use std::{ffi::OsStr, path::{Path, PathBuf}};

pub mod sound_archive_mode;
pub mod sound_archive;
pub mod aax {
    pub mod archive;
    pub mod entry;
    pub mod loop_flag;
}
pub mod adpcm_wii {
    pub mod archive;
    pub mod entry;
    pub mod loop_flag;
}
pub mod swlpcm {
    // pub mod archive;
    // pub mod entry;
    // pub mod loop_flag;
}
pub mod cpk {
    pub mod data {
        // This will be used as a standalone CPK archive
    }
    pub mod sound {
        pub mod archive;
        pub mod mode;
        pub mod entry;
    }
}
pub mod csb {
    pub mod common {
        pub mod cri_csb_aax_archive;
        pub mod cri_csb_aax_sound_element;
        pub mod cri_csb_adpcm_wii_archive;
        pub mod cri_csb_adpcm_wii_sound_element;
        pub mod cri_csb_archive;
    }
    pub mod games {
        pub mod black_knight {
            pub mod archive;
        }
        pub mod episodes {
            pub mod archive;
        }
    }
}

pub mod games {
    pub mod episodes {
        pub mod extractor;
        pub mod creator;
    }
    pub mod black_knight {
        pub mod extractor;
        pub mod creator;
        pub mod patcher;
    }
}

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn find_related_file(path_to_search: &Path, extension: &OsStr) -> Option<PathBuf> {
    let path = path_to_search.with_added_extension(extension);
    path.exists().then(|| path.to_path_buf())
}

pub fn find_related_cpk_of_csb(csb_path: &Path) -> Option<PathBuf> {
    let base_directory = csb_path.parent().unwrap();
    let path_to_search = base_directory.join(csb_path.file_stem().unwrap());

    find_related_file(&path_to_search, OsStr::new("CPK"))
        .or_else(|| find_related_file(&path_to_search, OsStr::new("cpk")))
}

pub fn find_related_csb_of_dir(dir_path: &Path) -> Option<PathBuf> {
    find_related_file(&dir_path, OsStr::new("CSB"))
        .or_else(|| find_related_file(&dir_path, OsStr::new("csb")))
}