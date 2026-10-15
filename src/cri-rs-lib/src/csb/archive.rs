use std::{fs, path::Path};

use common_binary::error::CommonBinaryError::{self, SeeConsole};
use cri_table::table::{CriTable, UTF_SIGNATURE};
use crate::{aax::archive::CriAaxArchive, cpk::archive::CriCpkArchive, find_cpk};

pub struct CriCsbArchive {
    pub sound: CriSoundArchive,
}

pub struct CriSoundArchive {
    pub data: Vec<CriSoundData>,
}

#[derive(Debug)]
pub enum CriSoundData {
    Aax(CriAaxArchive),
    Stream(CriCpkArchive),
}

fn get_sound_data(sdl_table: &CriTable, cpk: Option<CriCpkArchive>) -> Result<Vec<CriSoundData>, CommonBinaryError> {
    let is_streaming = sdl_table.get_field("stmflg").unwrap().to_bool(0).unwrap();
    if sdl_table.get_field("fmt").unwrap().to_bool(0).unwrap() {
        eprintln!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
        return Err(SeeConsole());
    }

    match (is_streaming, cpk) {
        (true, None) => {
            eprintln!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
            return Err(SeeConsole());
        },
        (true, Some(cpk)) => {
            return Ok(vec![CriSoundData::Stream(cpk)]);
        }
        (false, _) => {
            let mut sound_entries = Vec::with_capacity(sdl_table.row_count());
            for sdl_row_index in 0..sdl_table.row_count() {
                let sdl_name = sdl_table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();                
                let aax_table = sdl_table.get_field("data").unwrap().to_table(sdl_row_index).unwrap();
                sound_entries.push(CriSoundData::Aax(
                    CriAaxArchive::read(aax_table, &sdl_name).unwrap()
                ));
            }
            return Ok(sound_entries);
        },
    };
}

struct CsbFields {
    pub sound_element: Option<CriTable>,
}

fn get_csb_fields(csb_table: &CriTable) -> CsbFields {
    // Maybe redo as a builder pattern?
    let mut sound_element = None;
    // /Maybe redo as a builder pattern?

    for row_index in 0..csb_table.row_count() {
        match csb_table.get_field("name").unwrap().to_string(row_index).unwrap().as_str() {
            "SOUND_ELEMENT" => sound_element = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            _ => (),
        }
    }

    CsbFields {
        sound_element,
    }
}

impl CriCsbArchive {
    pub fn read(path: &Path) -> Result<CriCsbArchive, CommonBinaryError> {
        let base_directory = path.parent().unwrap();
        let output_directory_name = base_directory.join(path.file_stem().unwrap());
        let file_content = fs::read(path).unwrap();
        let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();
        
        let cpk_path = find_cpk(&output_directory_name.to_string_lossy());
        let cpk_data = cpk_path.map(|path| fs::read(path).unwrap());
        let cpk = cpk_data.map(|data| CriCpkArchive::read(&data, 0).unwrap());

        // Redo as builder pattern?
        let mut sound_data = None;
        // /Redo as builder pattern?

        let fields = get_csb_fields(&csb_table);

        if let Some(sdl_table) = fields.sound_element {
            sound_data = Some(get_sound_data(&sdl_table, cpk).unwrap());
        };

        return Ok(
            CriCsbArchive {
                sound: CriSoundArchive {
                    data: sound_data.unwrap(),
                },
            }
        );
    }
}