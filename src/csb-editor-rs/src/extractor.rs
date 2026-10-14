use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::archive::CriAaxArchive, csb::archive::CriCsbArchive};
use cri_table::table::{CriTable, UTF_SIGNATURE};

pub fn extract_csb(path: &str) {
    let path = Path::new(path);
    let file_content = fs::read(path).unwrap();
    let csb = CriCsbArchive::read(path).unwrap();
    let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for row_index in 0..csb_table.row_count() {
        if csb_table.get_field("name").unwrap().to_string(row_index).unwrap() == "SOUND_ELEMENT" {
            let sdl_table = csb_table.get_field("utf").unwrap().to_table(row_index).unwrap();
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
                                let aax_archive = &cpk_entry.aax_archive;
                                for entry in &aax_archive.entries {
                                    fs::write(
                                        destination_path.clone().join(entry.flag.to_file_name()),
                                        &entry.data,
                                    ).unwrap();
                                }
                            }
                            None => ()
                        }
                    },
                    None => {
                        let aax_table = sdl_table.get_field("data").unwrap().to_table(sdl_row_index).unwrap();
                        let aax_archive = CriAaxArchive::read(aax_table.to_owned()).unwrap();

                        for entry in aax_archive.entries {
                            fs::write(
                                destination_path.join(entry.flag.to_file_name()),
                                &entry.data,
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