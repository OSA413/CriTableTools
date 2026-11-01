use std::{fs::{self, create_dir_all}, path::Path};

use cri_table::table::{CriTable, UTF_SIGNATURE};

use crate::{aax::archive::CriAaxArchive, adpcm_wii::archive::CriAdpcmWiiArchive, csb::common::{adpcm_wii_archive::CriCsbAdpcmWiiArchive, sound_element_metainfo::CriCsbSoundElementMetaInfo}};

fn extract_aax(entry_dir_name: &Path, aax: &CriAaxArchive) {
    for entry in &aax.entries {
        fs::write(
            entry_dir_name.join(entry.flag.to_file_name().with_added_extension("adx")),
            &entry.data,
        ).unwrap();
    }
}

fn extract_dps(dsp: &CriAdpcmWiiArchive, base_dir: &Path) {
    for entry in &dsp.entries {
        let entry_dir_name = &base_dir.join(&dsp.name);
        create_dir_all(entry_dir_name).unwrap();
        fs::write(
            entry_dir_name.join(entry.loop_flag.to_file_name().with_added_extension("dsp")),
            [
                entry.header.clone(),
                entry.data.clone(),
            ].concat(),
        ).unwrap();
        
        fs::write(
            entry_dir_name.join("sound_element.json"),
            serde_json::to_string(&CriCsbSoundElementMetaInfo {                
                original_string_pool: dsp.original_string_pool.clone(),
                name: dsp.name.clone(),
                fmt: dsp.fmt,
                nch: dsp.nch,
                stmflg: dsp.stmflg,
                sfreq: dsp.sfreq,
                nsmpl: dsp.nsmpl,
            }).unwrap(),
        ).unwrap();
    }
}

pub fn extract(path: &Path) {
    let content = fs::read(path).unwrap();
    let table = CriTable::read(&content, None, UTF_SIGNATURE).unwrap();
    let csb = CriCsbAdpcmWiiArchive::from_table(&table).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for sound_entry in &csb.sound_element.0 {
        extract_dps(&sound_entry.data, &output_directory_name);
    }

    fs::write(
        output_directory_name.join("cue.json"),
        serde_json::to_string(&csb.cue.0).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("synth.json"),
        serde_json::to_string(&csb.synth.0).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("table_info.json"),
        serde_json::to_string(&csb.info.0).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("cue.json"),
        serde_json::to_string(&csb.cue.0).unwrap(),
    ).unwrap();

    fs::write(
        output_directory_name.join("synth.json"),
        serde_json::to_string(&csb.synth.0).unwrap(),
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
        serde_json::to_string(&csb.voice_limit_group.0).unwrap(),
    ).unwrap();

    println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy());
}