use std::{fs, path::Path};

use common_utils::walk_dir::walk_dir;
use cri_table::table::{CriTable, UTF_SIGNATURE};

use crate::{adpcm_wii::{archive::CriCsbAdpcmWiiArchive, entry::CriAdpcmWiiEntry}, csb::{common::archive::CriCsbArchive, creator::parse_entry, sound_element::{CriCsbSoundElement, sound_element_metainfo::CriCsbSoundElementMetaInfo}}, sound_archive::CriCsbSoundArchiveEnum, sound_archive_mode::CriSoundArchiveFormat, traits::{FromCriTable, ToCriTable}};

pub fn patch(path: &Path) {
    let path_path = Path::new(path);
    let content = fs::read(path).unwrap();
    let table = CriTable::read(&content, None, UTF_SIGNATURE).unwrap();
    let mut csb = CriCsbArchive::from_table(&table).unwrap();
    let dir_path = &path_path.with_extension("");

    let fmt = csb.sound_element.0.first().unwrap().fmt;
    let expected_format = match fmt {
        0 => CriSoundArchiveFormat::Adx,
        4 => CriSoundArchiveFormat::Dsp,
        // 5 => CriSoundArchiveFormat::Wav,
        _ => panic!("The given CSB file contains an audio file which is not of supported format. Only CSB files with ADX or DSP files are supported."),
    };

    let expected_extension = match expected_format {
        CriSoundArchiveFormat::Adx => ".adx",
        CriSoundArchiveFormat::Dsp => ".dsp",
        CriSoundArchiveFormat::Wav => todo!(),
    };

    let files = walk_dir(dir_path, None);



    for entry in files.iter().filter(|entry| entry.to_string_lossy().to_lowercase().ends_with(expected_extension)) {
        let (root_entry, loop_flag) = match parse_entry(entry, &expected_format) {
            Some(parsed_entry) => parsed_entry,
            None => continue,
        };
        
        let relative_name = root_entry.root_dir_path
            .strip_prefix(dir_path).unwrap()
            .to_string_lossy().replace('\\', "/");

        let entry_position = csb.sound_element.0.iter().position(|x| x.name == relative_name).unwrap();

        let old_entry = &csb.sound_element.0[entry_position];
        let old_entry_sound_element = &old_entry.data;

        let sound_element_meta_data = fs::read_to_string(root_entry.root_dir_path.join("sound_element.json"))
            .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data))
            .unwrap();

        let data = fs::read(entry).unwrap();

        let data = match expected_format {
            CriSoundArchiveFormat::Adx => todo!(),
            CriSoundArchiveFormat::Dsp => CriCsbSoundArchiveEnum::AdpcmWii(
                CriCsbAdpcmWiiArchive {
                    original_string_pool: root_entry.meta_info.original_string_pool.clone(),
                    name: relative_name.clone(),
                    sfreq: root_entry.meta_info.sfreq,
                    fmt: 4,
                    stmflg: 0,
                    nsmpl: root_entry.meta_info.nsmpl,
                    nch: root_entry.meta_info.nch,        
                    entries: vec![
                        CriAdpcmWiiEntry {
                            loop_flag: loop_flag,
                            sfreq: root_entry.meta_info.sfreq,
                            nsmpl: root_entry.meta_info.nsmpl,
                            nch: root_entry.meta_info.nch,
                            // TODO: turns out there can be two headers in one header,
                            // so this import is incorrect (as well as the extraction, probably)
                            header: data[..0x60].to_vec(),
                            data: data[0x60..].to_vec(),
                        }
                    ],
                }
            ),
            CriSoundArchiveFormat::Wav => todo!(),
        };

        let new_entry = CriCsbSoundElement {
            original_string_pool: root_entry.meta_info.original_string_pool,
            name: relative_name,
            data,
            fmt: old_entry.fmt,
            nch: old_entry.nch,
            stmflg: old_entry.stmflg,
            sfreq: old_entry.sfreq,
            nsmpl: old_entry.nsmpl,
        };

        csb.sound_element.0[entry_position] = new_entry;
    }

    // csb.cue = fs::read_to_string(path.join("cue.json"))
    //     .map(|data| serde_json::from_str::<Vec<CriCsbAdpcmWiiCue>>(&data))
    //     .unwrap()
    //     .unwrap();

    // csb.synth = fs::read_to_string(path.join("synth.json"))
    //     .map(|data| serde_json::from_str::<Vec<CriCsbAdpcmWiiSynth>>(&data))
    //     .unwrap()
    //     .unwrap();

    // csb.info = fs::read_to_string(path.join("table_info.json"))
    //     .map(|data| serde_json::from_str::<Vec<CriCsbAdpcmWiiInfo>>(&data))
    //     .unwrap()
    //     .unwrap();

    // let isaac_entries = walk_dir(path, Some(OsStr::new("isaac.json")));

    // for entry in isaac_entries {
    //     let isaac_info = fs::read_to_string(entry)
    //         .map(|data| serde_json::from_str::<CriCsbAdpcmWiiIsaac>(&data))
    //         .unwrap()
    //         .unwrap();
    
    //     csb.isaac.push(isaac_info);
    // }
    
    // csb.voice_limit_group = fs::read_to_string(path.join("voice_limit_group.json"))
    //     .map(|data| serde_json::from_str::<Vec<CriCsbAdpcmWiiVoiceLimitGroup>>(&data))
    //     .unwrap()
    //     .unwrap();

    fs::write(path.with_added_extension("patched"), csb.to_table().unwrap().write().unwrap()).unwrap();
}