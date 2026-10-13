use std::{fs, path::{Path, PathBuf}};

use common_binary::error::CommonBinaryError::{self, SeeConsole};
use cri_table::table::{CriTable, UTF_SIGNATURE};
use crate::cpk::archive::CriCpkArchive;

pub struct Csb {
    table: CriTable,
    stream: Option<CriCpkArchive>,
}

fn find_cpk(path_to_search: &str) -> Option<PathBuf> {
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

impl Csb {
    pub fn read_file(path: &Path) -> Result<Csb, CommonBinaryError> {
        let base_directory = path.parent().unwrap();
        let output_directory_name = base_directory.join(path.file_stem().unwrap());

        let cpk_path = find_cpk(&output_directory_name.to_string_lossy());

        let file_content = fs::read(path).unwrap();

        let reader = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();

        for row_index in 0..reader.table.row_count() {
            if reader.table.get_field("name").unwrap().to_string(row_index).unwrap() == "SOUND_ELEMENT" {
                let sdl_reader = CriTable::read(
                     reader.table.get_field("utf").unwrap().to_data(row_index).unwrap(),
                    Some(0),
                    UTF_SIGNATURE,
                ).unwrap();

                for sdl_row_index in 0..sdl_reader.table.row_count()
                {
                    if sdl_reader.table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap() != 0 {
                        eprintln!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                        return Err(SeeConsole());
                    }

                    let streaming = sdl_reader.table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap() != 0;

                    if streaming && cpk_path.is_none() {
                        eprintln!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
                        return Err(SeeConsole());
                    } else if streaming && cpk_path.is_some() {
                        let cpk_source = fs::read(cpk_path.as_ref().unwrap()).unwrap();
                        let cpk = CriCpkArchive::read(&cpk_source, 0).unwrap();
                        return Ok(Csb { table: reader.table, stream: Some(cpk) });
                    } else {
                        return Ok(Csb { table: reader.table, stream: None });
                    }
                }
            }
        }
        
        return Ok(Csb { table: reader.table, stream: None });
    }
}