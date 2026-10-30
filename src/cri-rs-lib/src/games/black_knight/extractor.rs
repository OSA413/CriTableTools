use std::{fs::{self, create_dir_all}, path::Path};

use crate::{adpcm_wii::archive::CriAdpcmWiiArchiveWrapper, csb::{common::{cri_csb_adpcm_wii_archive::CriCsbAdpcmWiiArchive, cri_csb_adpcm_wii_sound_element::CriCsbAdpcmWiiSoundElementData}, games::black_knight::archive::CriCsbSoundElementMetaInfo}};

fn extract_dps(dps: &CriAdpcmWiiArchiveWrapper, base_dir: &Path) {
    for entry in &dps.entries {
        for inner_entry in &entry.entries {
            let entry_dir_name = &base_dir.join(&entry.name);
            create_dir_all(entry_dir_name).unwrap();
            fs::write(
                entry_dir_name.join(inner_entry.loop_flag.to_file_name()),
                [
                    inner_entry.header.clone(),
                    inner_entry.data.clone(),
                ].concat(),
            ).unwrap();
            
            fs::write(
                entry_dir_name.join("sound_element.json"),
                serde_json::to_string(&CriCsbSoundElementMetaInfo {                
                    original_string_pool: entry.original_string_pool.clone(),
                    name: entry.name.clone(),
                    fmt: entry.fmt,
                    nch: entry.nch,
                    stmflg: entry.stmflg,
                    sfreq: entry.sfreq,
                    nsmpl: entry.nsmpl,
                }).unwrap(),
            ).unwrap();
        }
    }
}

pub fn extract_csb(path: &Path) {
    let csb = CriCsbAdpcmWiiArchive::read(path).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for sound_entry in &csb.sound_element.0 {
        match &sound_entry.data {
            CriCsbAdpcmWiiSoundElementData::AdpcmWii(dsp) =>
                extract_dps(dsp, &output_directory_name),
        };
    }

    if let Some(first) = &csb.sound_element.0.first() {
        let format = match &first.data {
            CriCsbAdpcmWiiSoundElementData::AdpcmWii(archive_wrapper) => &archive_wrapper.entries[0].format,
        };
        fs::write(
            output_directory_name.join("format.json"),
            serde_json::to_string(format).unwrap(),
        ).unwrap();
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

    for isaac in &csb.isaac.0 {
        let file_path = output_directory_name.join(format!("{}.isaac.json", isaac.ptname));
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

    println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy());
}