use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{aax::archive::CriAaxArchive, csb::{common::{cri_csb_aax_archive::CriCsbAaxArchive, cri_csb_aax_sound_element::CriCsbAaxSoundElementData}, games::black_knight::archive::CriCsbSoundElementMetaInfo}, sound_archive::CriSoundArchiveEnum};

fn extract_aax(entry_dir_name: &Path, aax: &CriAaxArchive) {
    for entry in &aax.entries {
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
    let csb = CriCsbAaxArchive::read(path).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for entry in &csb.sound_element {
        let entry_dir_name = &output_directory_name.join(&entry.name);
        create_dir_all(entry_dir_name).unwrap();
        fs::write(
            entry_dir_name.join("sound_element.json"),
            serde_json::to_string(&CriCsbSoundElementMetaInfo {
                name: entry.name.clone(),
                fmt: entry.fmt,
                nch: entry.nch,
                stmflg: entry.stmflg,
                sfreq: entry.sfreq,
                nsmpl: entry.nsmpl,
            }).unwrap(),
        ).unwrap();
        match &entry.data {
            CriCsbAaxSoundElementData::Aax(aax) => 
                extract_aax(
                    &entry_dir_name,
                    &aax,
                ),
            CriCsbAaxSoundElementData::Stream(cpk) => {
                for entry in &cpk.entries {
                    extract_cpk(&entry.directory_name, match &entry.aax_archive.data {
                        CriSoundArchiveEnum::Aax(cri_aax_archive) => &cri_aax_archive,
                        _ => panic!("wanna sleep"),
                    }, &output_directory_name.join(&entry.directory_name))
                }
            }
        }
    }

    fs::write(
        output_directory_name.join("cue.json"),
        serde_json::to_string(&csb.cue).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("synth.json"),
        serde_json::to_string(&csb.synth).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("table_info.json"),
        serde_json::to_string(&csb.info).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("cue.json"),
        serde_json::to_string(&csb.cue).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("synth.json"),
        serde_json::to_string(&csb.synth).unwrap(),
    ).unwrap();

    for isaac in &csb.isaac {
        let file_path = output_directory_name.join(&format!("{}.isaac.json", isaac.ptname));
        create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(
            &file_path,
            serde_json::to_string(&isaac).unwrap(),
        ).unwrap();
    }

    let file_path = output_directory_name.join("voice_limit_group.json");
    fs::write(
        &file_path,
        serde_json::to_string(&csb.voice_limit_group).unwrap(),
    ).unwrap();

    match &csb.sound_element.iter().find(|sound_entry| match sound_entry.data {
        CriCsbAaxSoundElementData::Aax(_) => false,
        CriCsbAaxSoundElementData::Stream(_) => true,
    }) {
        Some(_) => println!("Extracted CSB and CPK files: \"{}\"", output_directory_name.to_string_lossy()),
        None => println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy()),
    }
}