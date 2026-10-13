use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::{archive::CriAaxArchive, entry_flag::CriAaxEntryFlag}, cpk::archive::CriCpkArchive, find_cpk};
use cri_table::table::{CriTable, UTF_SIGNATURE};

pub fn extract_csb(path: &str) {
    let path = Path::new(path);

    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    let mut cpk_archive: Option<CriCpkArchive> = None;
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
                    panic!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                }

                let streaming = sdl_reader.table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap() != 0;

                if streaming && cpk_path.is_none() {
                    panic!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
                } else if streaming && cpk_path.is_some() && cpk_archive.is_none() {
                    let cpk_source = fs::read(cpk_path.as_ref().unwrap()).unwrap();
                    cpk_archive = Some(CriCpkArchive::read(&cpk_source, 0).unwrap());
                }

                let sdl_name = sdl_reader.table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();
                let destination_path = output_directory_name.join(&sdl_name);
                create_dir_all(&destination_path).unwrap();

                if streaming {
                    let cpk_entry = cpk_archive.as_ref().unwrap().get_by_path(&sdl_name.into_boxed_str());

                    match cpk_entry {
                        Some(cpk_entry) => {
                            let aax_table = &cpk_entry.table;
                            for row_index in 0..aax_table.row_count() {
                                let flag = CriAaxEntryFlag::from(aax_table.get_field("lpflg").unwrap().to_u8(row_index).unwrap());
                                fs::write(
                                    destination_path.clone().join(flag.to_file_name()),
                                    aax_table.get_field("data").unwrap().to_data(row_index).unwrap(),
                                ).unwrap();
                            }
                        }
                        None => ()
                    }
                } else {
                    let aax_reader = CriTable::read(
                        sdl_reader.table.get_field("data").unwrap().to_data(row_index).unwrap(),
                        Some(0),
                        UTF_SIGNATURE,
                    ).unwrap();
                    let aax_archive = CriAaxArchive::read(aax_reader.table).unwrap();

                    for aax_row_index in 0..aax_archive.table.row_count() {
                        let flag = CriAaxEntryFlag::from(aax_archive.table.get_field("lpflg").unwrap().to_u8(aax_row_index).unwrap());
                        let data = aax_archive.table.get_field("data").unwrap().to_data(aax_row_index).unwrap();
                        fs::write(
                            destination_path.join(flag.to_file_name()),
                            &data,
                        ).unwrap();
                    }
                }
            }
        }
    }
    match cpk_archive {
        Some(_) => println!("Extracted CSB and CPK files: \"{}\"", output_directory_name.to_string_lossy()),
        None => println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy()),
    }
}