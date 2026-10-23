use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::archive::CriAaxArchive, csb::games::other::archive::{CriCsbArchive, CriCsbSoundElementData}, sound_archive::CriSoundArchiveEnum};

fn extract_aax(entry_dir_name: &str, aax: &CriAaxArchive, base_dir: &Path) {
    for entry in &aax.entries {
        let entry_dir_name = &base_dir.join(&entry_dir_name);
        create_dir_all(entry_dir_name).unwrap();
        fs::write(
            entry_dir_name.join(entry.flag.to_file_name()),
            &entry.data,
        ).unwrap();
    }
}

fn extract_cpk(entry_dir_name: &str, aax: &CriAaxArchive, base_dir: &Path) {
    todo!("I forgot what to do here :skull:");
    for entry in &aax.entries {
        let entry_dir_name = &base_dir.join(&entry_dir_name);
        create_dir_all(entry_dir_name).unwrap();
        fs::write(
            entry_dir_name.join(entry.flag.to_file_name()),
            &entry.data,
        ).unwrap();
    }
}

pub fn extract_csb(path: &Path) {
    let csb = CriCsbArchive::read(path).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for sound_entry in &csb.sound_element {
        match &sound_entry.data {
            CriCsbSoundElementData::Aax(aax) => extract_aax(&sound_entry.name.clone(), &aax, &output_directory_name),
            CriCsbSoundElementData::Stream(cpk) => {
                for entry in &cpk.entries {
                    extract_cpk(&entry.directory_name, match &entry.aax_archive.data {
                        CriSoundArchiveEnum::Aax(cri_aax_archive) => cri_aax_archive,
                        _ => panic!("wanna sleep"),
                    }, &output_directory_name.join(&entry.directory_name))
                }
            }
        }
    }

    match &csb.sound_element.iter().find(|sound_entry| match sound_entry.data {
        CriCsbSoundElementData::Aax(_) => false,
        CriCsbSoundElementData::Stream(_) => true,
    }) {
        Some(_) => println!("Extracted CSB and CPK files: \"{}\"", output_directory_name.to_string_lossy()),
        None => println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy()),
    }
}