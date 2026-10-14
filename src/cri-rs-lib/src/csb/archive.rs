use std::{fs, path::Path};

use common_binary::error::CommonBinaryError::{self, SeeConsole};
use cri_table::table::{CriTable, UTF_SIGNATURE};
use crate::{cpk::archive::CriCpkArchive, find_cpk};

pub struct CriCsbArchive {
    pub table: CriTable,
    pub stream: Option<CriCpkArchive>,
}

impl CriCsbArchive {
    pub fn read(path: &Path) -> Result<CriCsbArchive, CommonBinaryError> {
        let base_directory = path.parent().unwrap();
        let output_directory_name = base_directory.join(path.file_stem().unwrap());
        let cpk_path = find_cpk(&output_directory_name.to_string_lossy());
        let file_content = fs::read(path).unwrap();
        let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();

        for row_index in 0..csb_table.row_count() {
            if csb_table.get_field("name").unwrap().to_string(row_index).unwrap() == "SOUND_ELEMENT" {
                let sdl_table = &csb_table.get_field("utf").unwrap().to_table(row_index).unwrap();
                for sdl_row_index in 0..sdl_table.row_count() {
                    if sdl_table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap() != 0 {
                        eprintln!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                        return Err(SeeConsole());
                    }

                    let streaming = sdl_table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap() != 0;

                    match (streaming, cpk_path) {
                        (true, None) => {
                            eprintln!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
                            return Err(SeeConsole());
                        },
                        (true, Some(cpk_path)) => {
                            let cpk = CriCpkArchive::read(&fs::read(cpk_path).unwrap(), 0).unwrap();
                            return Ok(CriCsbArchive { table: csb_table, stream: Some(cpk) })
                        },
                        (false, _) => return Ok(CriCsbArchive { table: csb_table, stream: None }),
                    }
                }
            }
        }
        
        return Ok(CriCsbArchive { table: csb_table, stream: None });
    }
}