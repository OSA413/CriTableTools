use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::{archive::CriAaxArchive, entry_flag::CriAaxEntryFlag}, csb::archive::CriCsbArchive};
use cri_table::table::{CriTable, UTF_SIGNATURE};

pub fn extract_csb(path: &str) {
    let path = Path::new(path);
    let file_content = fs::read(path).unwrap();
    let csb = CriCsbArchive::read(path).unwrap();
    let csb_reader = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for row_index in 0..csb_reader.table.row_count() {
        if csb_reader.table.get_field("name").unwrap().to_string(row_index).unwrap() == "SOUND_ELEMENT" {
            let sdl_source = csb_reader.get_field_source(&file_content, "utf", row_index);
            let sdl_reader = CriTable::read(
                &sdl_source,
                Some(0),
                *b"@UTF",
            ).unwrap();
            let sdl_table = &sdl_reader.table;
            for sdl_row_index in 0..sdl_table.row_count() {
                if sdl_table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap() != 0 {
                    panic!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                }

                let sdl_name = sdl_table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();
                let destination_path = output_directory_name.join(&sdl_name);
                create_dir_all(&destination_path).unwrap();

                match &csb.stream {
                    Some(cpk_archive) => {
                        match cpk_archive.get_by_path(&sdl_name.into_boxed_str()) {
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
                    },
                    None => {
                        let aax_source = &sdl_reader.get_field_source(&sdl_source, "data", sdl_row_index);
                        let aax_reader = CriTable::read(
                            aax_source,
                            Some(0),
                            UTF_SIGNATURE
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
                    },
                }
            }
        }
    }
    match csb.stream {
        Some(_) => println!("Extracted CSB and CPK files: \"{}\"", output_directory_name.to_string_lossy()),
        None => println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy()),
    }
}