use std::{fs::{self, create_dir_all}, path::{Path, PathBuf}};

use sonic_audio_lib_rs::archives::cri::{aax::{archive::CriAaxArchive, entry_flag::CriAaxEntryFlag}, cpk::archive::CriCpkArchive, mw::table::table::CriTable};

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

pub fn extract_csb(path: &str) {
    let path = Path::new(path);

    let extension = match path.extension() {
        Some(extension) => extension.to_ascii_lowercase(),
        None => panic!("No extension found"),
    };

    if extension == "csb" {
        println!("Detected CSB extension, trying extracting");
        let base_directory = path.parent().unwrap();
        let output_directory_name = base_directory.join(path.file_stem().unwrap());

        let mut cpk_archive: Option<CriCpkArchive> = None;
        let cpk_path = find_cpk(&output_directory_name.to_string_lossy());

        let file_content = fs::read(path).unwrap();

        let reader = CriTable::read(&file_content, Some(0), [0x40, 0x55, 0x54, 0x46]).unwrap();

        for row_index in 0..reader.row_count() {
            if reader.table.get_field("name").unwrap().to_string(row_index).unwrap() == "SOUND_ELEMENT" {
                let sdl_source = reader.get_field_source(&file_content, row_index, "utf");
                let sdl_reader = CriTable::read(
                    &sdl_source,
                    Some(0),
                    *b"@UTF",
                ).unwrap();

                for sdl_row_index in 0..sdl_reader.row_count()
                {
                    if sdl_reader.table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap() != 0 {
                        panic!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                    }

                    let streaming = sdl_reader.table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap() != 0;

                    if streaming && cpk_path.is_none() {
                        panic!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
                    } else if streaming && cpk_path.is_some() && cpk_archive.is_none() {
                        println!("The CSB got a CPK, trying to read it also");
                        let cpk_source = Some(fs::read(cpk_path.as_ref().unwrap()).unwrap());
                        cpk_archive = Some(CriCpkArchive::read(&cpk_source.as_ref().unwrap(), 0).unwrap());
                    }

                    let sdl_name = sdl_reader.table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();
                    let destination_path = output_directory_name.join(&sdl_name);
                    create_dir_all(&destination_path).unwrap();

                    if streaming {
                        let cpk_entry = cpk_archive.as_ref().unwrap().get_by_path(&sdl_name.into_boxed_str());

                        match cpk_entry {
                            Some(cpk_entry) => {
                                let aax_source = &cpk_entry.value;
                                let aax_archive = CriAaxArchive::read(aax_source, Some(0)).unwrap();

                                for entry in aax_archive.entries {
                                    let adx_file_name = match entry.flag {
                                        CriAaxEntryFlag::Intro => "Intro.adx",
                                        CriAaxEntryFlag::Loop => "Loop.adx",
                                    };

                                    fs::write(
                                        destination_path.clone().join(adx_file_name),
                                        &entry.data,
                                    ).unwrap();
                                }
                            }
                            None => ()
                        }
                    } else {
                        let aax_source = sdl_reader.get_field_source(&sdl_source, sdl_row_index, "data");
                        let aax_archive = CriAaxArchive::read(&aax_source, Some(0)).unwrap();

                        for entry in aax_archive.entries {
                            let adx_file_name = match entry.flag {
                                CriAaxEntryFlag::Intro => "Intro.adx",
                                CriAaxEntryFlag::Loop => "Loop.adx",
                            };

                            fs::write(
                                destination_path.join(adx_file_name),
                                &entry.data,
                            ).unwrap();
                        }
                    }
                }
            }
        }
        println!("Extraction complete!");
    }
}