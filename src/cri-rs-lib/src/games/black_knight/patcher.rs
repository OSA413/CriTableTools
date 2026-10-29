use std::{fs, path::Path};

use common_utils::walk_dir::walk_dir;

use crate::{adpcm_wii::{archive::{CriAdpcmWiiArchive, CriAdpcmWiiArchiveWrapper}, entry::CriAdpcmWiiEntry}, csb::common::{cri_csb_adpcm_wii_archive::CriCsbAdpcmWiiArchive, cri_csb_adpcm_wii_sound_element::{CriCsbAdpcmWiiSoundElement, CriCsbAdpcmWiiSoundElementData}}, games::black_knight::creator::parse_entry};

pub fn patch(path: &Path) {
    let path_path = Path::new(path);
    let mut csb = CriCsbAdpcmWiiArchive::read(path).unwrap();
    fs::write(format!("{}.recreated_csb", path.to_string_lossy()), csb.write().unwrap()).unwrap();
    let dir_path = &path_path.with_extension("");

    let files = walk_dir(dir_path, None);

    for entry in files.iter().filter(|entry| entry.to_string_lossy().to_lowercase().ends_with(".dsp")) {
        let (root_entry, loop_flag) = match parse_entry(entry) {
            Some(parsed_entry) => parsed_entry,
            None => continue,
        };
        
        let relative_name = root_entry.root_dir_path
            .strip_prefix(dir_path).unwrap()
            .to_string_lossy().replace('\\', "/");

        let entry_position = csb.sound_element.0.iter().position(|x| x.name == relative_name).unwrap();

        let old_entry = &csb.sound_element.0[entry_position];
        let CriCsbAdpcmWiiSoundElementData::AdpcmWii(old_entry_sound_element) = &old_entry.data;

        // let sound_element_meta_data = fs::read_to_string(root_entry.root_dir_path.join("sound_element.json"))
        //     .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data));

        let data = fs::read(entry).unwrap();

        let new_entry = CriCsbAdpcmWiiSoundElement {
            original_string_pool: Vec::new(),//TODO
            name: relative_name,
            data: CriCsbAdpcmWiiSoundElementData::AdpcmWii(
                CriAdpcmWiiArchiveWrapper {
                    original_string_pool: old_entry.original_string_pool.clone(),
                    entries: vec![
                        CriAdpcmWiiArchive {
                            original_string_pool: old_entry_sound_element.original_string_pool.clone(),
                            fmt: old_entry_sound_element.entries[0].fmt,
                            name: old_entry_sound_element.entries[0].name.clone(),
                            nch: old_entry_sound_element.entries[0].nch,
                            nsmpl: old_entry_sound_element.entries[0].nsmpl,
                            sfreq: old_entry_sound_element.entries[0].sfreq,
                            stmflg: old_entry_sound_element.entries[0].stmflg,
                            entries: vec![
                                CriAdpcmWiiEntry {
                                    sfreq: root_entry.meta_info.sfreq,
                                    nsmpl: root_entry.meta_info.nsmpl,
                                    nch: root_entry.meta_info.nch,
                                    loop_flag,
                                    // TODO: turns out there can be two headers in one header,
                                    // so this import is incorrect (as well as the extraction, probably)
                                    header: data[..0x60].to_vec(),
                                    data: data[0x60..].to_vec(),
                                },
                            ],
                        }
                    ]
                },
            ),
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

    // fs::write(format!("{}.patched", path.to_string_lossy()), csb.write().unwrap()).unwrap();
}