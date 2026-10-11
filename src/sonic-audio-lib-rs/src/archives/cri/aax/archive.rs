use std::{ffi::OsStr, path::Path};

use common_binary::error::CommonBinaryError;

use crate::archives::cri::{aax::{archive_mode::CriAaxArchiveMode, entry::CriAaxEntry, entry_flag::CriAaxEntryFlag}, mw::table::table::CriTable};

pub struct CriAaxArchive {
    pub mode: CriAaxArchiveMode,
    pub entries: Vec<CriAaxEntry>,
}

impl CriAaxArchive {
    pub fn read(source: &[u8], pointer: Option<usize>) -> Result<CriAaxArchive, CommonBinaryError> {
        let reader = CriTable::read(source, pointer, [0x40, 0x55, 0x54, 0x46])?;
        let mode = match reader.table.name.as_str() {
            "AAX" => CriAaxArchiveMode::Adx,
            "SWLPCM" => CriAaxArchiveMode::Wav,
            "ADPCM_WII" => {
                eprintln!("DSP files aren't supported.");
                return Err(CommonBinaryError::SeeConsole());
            },
            value => {
                eprintln!("Unknown AAX type '{value}'");
                return Err(CommonBinaryError::SeeConsole());
            },
        };

        let mut entries = Vec::with_capacity(reader.row_count());
        for row_index in 0..reader.row_count() {
            entries.push(CriAaxEntry {
                flag: CriAaxEntryFlag::from(reader.table.get_field("lpflg").unwrap().to_u8(row_index)?),
                data: reader.get_field_source(&source, row_index, "data"),
            });
        }

        Ok(CriAaxArchive {
            mode,
            entries,
        })
    }

    // In the AaxArchive there can be only two entries as maximum: Intro and/or Loop
    // Maybe redo simpler?
    pub fn add(&mut self, item: CriAaxEntry) {
        if self.entries.len() == 2 || self.entries.iter().find(|entry| entry.flag == item.flag).is_some() {
            return;
        }
        self.entries.push(item)
    }

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
