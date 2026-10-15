use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::archive::CriAaxArchive, csb::archive::{CriCsbArchive, CriSoundData}};

fn extract_aax(aax: &CriAaxArchive, base_dir: &Path) {
    for entry in &aax.entries {
        let entry_dir_name = &base_dir.join(&aax.name);
        create_dir_all(entry_dir_name).unwrap();
        fs::write(
            entry_dir_name.join(entry.flag.to_file_name()),
            &entry.data,
        ).unwrap();
    }
}

pub fn extract_csb(path: &str) {
    let path = Path::new(path);
    let csb = CriCsbArchive::read(path).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for data_entry in &csb.sound.data {
        match data_entry {
            CriSoundData::Aax(aax) => extract_aax(aax, &output_directory_name),
            CriSoundData::Stream(cpk) => {
                for entry in &cpk.entries {
                    extract_aax(&entry.aax_archive, &output_directory_name.join(&entry.directory_name))
                }
            }
        };
    }

    match &csb.sound.data.iter().find(|data| match data {
        CriSoundData::Aax(_) => false,
        CriSoundData::Stream(_) => true,
    }) {
        Some(_) => println!("Extracted CSB and CPK files: \"{}\"", output_directory_name.to_string_lossy()),
        None => println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy()),
    }
}