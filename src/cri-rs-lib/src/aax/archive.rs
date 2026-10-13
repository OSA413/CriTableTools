use std::{ffi::OsStr, path::Path};

use common_binary::error::CommonBinaryError;
use cri_table::table::CriTable;

use crate::aax::archive_mode::CriAaxArchiveMode;

pub struct CriAaxArchive {
    pub mode: CriAaxArchiveMode,
    pub table: CriTable,
}

impl CriAaxArchive {
    pub fn read(table: CriTable) -> Result<CriAaxArchive, CommonBinaryError> {
        let mode = match table.name.as_str() {
            "AAX" => CriAaxArchiveMode::Adx,
            "SWLPCM" => CriAaxArchiveMode::Wav,
            "ADPCM_WII" => {
                eprintln!("DSP files are not supported.");
                return Err(CommonBinaryError::SeeConsole());
            },
            value => {
                eprintln!("Unknown AAX type [{value}]");
                return Err(CommonBinaryError::SeeConsole());
            },
        };

        Ok(CriAaxArchive {
            mode,
            table: table,
        })
    }

    // In the AaxArchive there can be only two entries as maximum: Intro and/or Loop
    // Maybe redo simpler?
    // TODO: move to csb-rs-lib
    // pub fn add(&mut self, item: CriAaxEntry) {
    //     if self.entries.len() == 2 || self.entries.iter().find(|entry| entry.flag == item.flag).is_some() {
    //         return;
    //     }
    //     self.entries.push(item)
    // }

    pub fn get_mode_extension(&self) -> &str {
        match self.mode {
            CriAaxArchiveMode::Adx => ".adx",
            CriAaxArchiveMode::Dsp => ".dsp",
            CriAaxArchiveMode::Wav => ".wav",
        }
    }

    pub fn set_mode_from_extension(&mut self, file_path: &OsStr) {
        self.mode = match Path::new(file_path).extension().unwrap().to_str().unwrap() {
            ".adx" => CriAaxArchiveMode::Adx,
            ".dsp" => CriAaxArchiveMode::Dsp,
            ".wav" => CriAaxArchiveMode::Wav,
            _ => panic!("This beat is not supported yet.")
        }
    }
}
