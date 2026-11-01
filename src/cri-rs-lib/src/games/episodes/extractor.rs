use std::path::Path;

use crate::csb::{self, common::{adpcm_wii_archive::CriCsbAdpcmWiiArchive}};

pub fn extract_csb(path: &Path) {
    csb::extractor::extract(path);
}