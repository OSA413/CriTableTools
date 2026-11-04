use std::{fs::{self, create_dir_all}, mem::MaybeUninit, path::Path, sync::{Arc, Mutex, RwLock}};

use cri_table::table::{CriTable, UTF_SIGNATURE};

use crate::{aax::archive::CriCsbAaxArchive, adpcm_wii::entry::CriAdpcmWiiEntry, cpk::archive::CriCpkArchive, csb::{common::archive::CriCsbArchive, sound_element::sound_element_metainfo::CriCsbSoundElementMetaInfo}, find_related_cpk_of_csb, sound_archive::CriCsbSoundArchiveEnum, traits::FromCriTable};

fn extract_aax(aax: &CriCsbAaxArchive, entry_dir_name: &Path) {
    for entry in &aax.entries {
        fs::write(
            entry_dir_name.join(entry.flag.to_file_name().with_added_extension("adx")),
            &entry.data,
        ).unwrap();
    }
}

fn extract_dps(dsp: &CriAdpcmWiiEntry, base_dir: &Path) {
    fs::write(
        base_dir.join(dsp.loop_flag.to_file_name().with_added_extension("dsp")),
        [
            dsp.header.clone(),
            dsp.data.clone(),
        ].concat(),
    ).unwrap();
    
    fs::write(
        base_dir.join("sound_element.json"),
        serde_json::to_string(&CriCsbSoundElementMetaInfo {
            original_string_pool: vec![],
            stmflg: 0,
            fmt: 4,
            nch: dsp.nch,
            sfreq: dsp.sfreq,
            nsmpl: dsp.nsmpl,
        }).unwrap(),
    ).unwrap();
}

pub fn extract(path: &Path) {
    let content = fs::read(path).unwrap();
    let table = CriTable::read(&content, None, UTF_SIGNATURE).unwrap();
    let csb = CriCsbArchive::from_table(&table).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());
    create_dir_all(&output_directory_name).unwrap();

    let cpk = find_related_cpk_of_csb(&path)
    .map(|cpk_path| fs::read(cpk_path).unwrap())
    .map(|cpk_content| CriCpkArchive::read(&cpk_content, 0).unwrap());
    let cpk_reference = Mutex::new(None);

    if cpk.is_some() {
        *cpk_reference.lock().unwrap() = cpk;
    }

    let cpk = cpk_reference;

    for sound_entry in &csb.sound_element.0 {
        let file_dir = output_directory_name.join(&sound_entry.name);
        create_dir_all(&file_dir).unwrap();
        match &sound_entry.data {
            CriCsbSoundArchiveEnum::Aax(archive)
                => extract_aax(&archive, &file_dir),
            CriCsbSoundArchiveEnum::AdpcmWii(archive)
                => extract_dps(&archive, &file_dir),
            CriCsbSoundArchiveEnum::Stream => {
                println!("{}", cpk.lock().unwrap().as_ref().unwrap().codec);
            },
        }
    }

    // I have not found the proper use of that, maybe I'll return it for a full re-creation purposes
    // Or I should improve the patching only?

    // fs::write(
    //     output_directory_name.join("cue.json"),
    //     serde_json::to_string(&csb.cue.0).unwrap(),
    // ).unwrap();

    // fs::write(
    //     output_directory_name.join("synth.json"),
    //     serde_json::to_string(&csb.synth.0).unwrap(),
    // ).unwrap();

    // fs::write(
    //     output_directory_name.join("table_info.json"),
    //     serde_json::to_string(&csb.info.0).unwrap(),
    // ).unwrap();

    // fs::write(
    //     output_directory_name.join("cue.json"),
    //     serde_json::to_string(&csb.cue.0).unwrap(),
    // ).unwrap();

    // fs::write(
    //     output_directory_name.join("synth.json"),
    //     serde_json::to_string(&csb.synth.0).unwrap(),
    // ).unwrap();

    // for isaac in &csb.isaac.0 {
    //     let file_path = output_directory_name.join(format!("{}.isaac.json", isaac.ptname));
    //     create_dir_all(file_path.parent().unwrap()).unwrap();
    //     fs::write(
    //         &file_path,
    //         serde_json::to_string(&isaac).unwrap(),
    //     ).unwrap();
    // }

    // let file_path = output_directory_name.join("voice_limit_group.json");
    // fs::write(
    //     &file_path,
    //     serde_json::to_string(&csb.voice_limit_group.0).unwrap(),
    // ).unwrap();

    println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy());
}