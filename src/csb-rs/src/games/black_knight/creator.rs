use std::{collections::HashMap, fs, io, path::{Path, PathBuf}};

use common_binary::error::CommonBinaryError;
use common_utils::walk_dir::walk_dir;
use cri_rs_lib::{adpcm_wii::{archive::{CriAdpcmWiiArchive, CriAdpcmWiiArchiveWrapper}, entry::CriAdpcmWiiEntry, loop_flag::CriDspLoopFlag}, csb::games::black_knight::archive::{CriCsbAdpcmWiiSoundElementData, CriCsbArchive, CriCsbCue, CriCsbInfo, CriCsbIsaac, CriCsbSoundElement, CriCsbSoundElementMetaInfo, CriCsbSynth, CriCsbVoiceLimitGroup}};
use serde::{Deserialize, Serialize};

static DATA_DSP: &'static str = "data.dsp";

pub struct ParsedDsp {
    root_path: String,
    root_dir_path: PathBuf,
    // Required as for now
    meta_info: DspMetaInfo,
    data_info: DspDataInfo,
}

#[derive(Serialize, Deserialize)]
pub struct DspMetaInfo {
    pub fmt: u8,
    pub name: String,
    pub nch: u8,
    pub nsmpl: u32,
    pub sfreq: u32,
    pub stmflg: u8,
}

#[derive(Serialize, Deserialize)]
pub struct DspDataInfo {
    pub sfreq: u32,
    pub nsmpl: u32,
    pub nch: u8,
    pub loop_flag: u8,
}

pub fn parse_entry(entry: &Path) -> Option<ParsedDsp> {
    let entry_string = entry.to_string_lossy();
    let entry_string_lowercase = entry_string.to_lowercase();
    let root_dir = entry_string.chars().take(entry_string.len() - DATA_DSP.len()).collect::<String>();
    let root_dir_path = Path::new(&root_dir);
    if entry_string_lowercase.ends_with(DATA_DSP) {
        return Some(ParsedDsp {
            root_path: root_dir.clone(),
            root_dir_path: root_dir_path.to_path_buf(),
            meta_info: fs::read_to_string(&root_dir_path.join("meta_info.json")).map(|data| serde_json::from_str::<DspMetaInfo>(&data).unwrap()).unwrap(),
            data_info: fs::read_to_string(&root_dir_path.join("data_info.json")).map(|data| serde_json::from_str::<DspDataInfo>(&data).unwrap()).unwrap(),
        })
    }
    None
}

pub fn create_from_dir(path: &Path) -> Result<(), CommonBinaryError> {
    let mut csb = CriCsbArchive {
        info: vec![],
        cue: vec![],
        synth: vec![],
        sound_element: vec![],
        isaac: vec![],
        voice_limit_group: vec![],
    };

    let files = walk_dir(path, None);

    let mut adpcm_wii_archive_pool: HashMap<String, (PathBuf, CriAdpcmWiiArchiveWrapper)> = HashMap::new();

    for entry in files.iter().filter(|entry| entry.ends_with(".dsp") || entry.ends_with(".DSP")) {
        let Some(root_entry) = parse_entry(&entry)
        else {
            continue
        };

        let data = fs::read(root_entry.root_dir_path.join(CriDspLoopFlag::Sound.to_file_name())).unwrap();

        let adpcm_wii_entry = CriAdpcmWiiArchive {
            fmt: root_entry.meta_info.fmt,
            name: root_entry.meta_info.name,
            nch: root_entry.meta_info.fmt,
            nsmpl: root_entry.meta_info.nsmpl,
            sfreq: root_entry.meta_info.sfreq,
            stmflg: root_entry.meta_info.fmt,
            entries: vec![
                CriAdpcmWiiEntry {
                    sfreq: root_entry.data_info.sfreq,
                    nsmpl: root_entry.data_info.nsmpl,
                    nch: root_entry.data_info.nch,
                    loop_flag: CriDspLoopFlag::Sound,
                    header: data[0..0x40].to_vec(),
                    data: data[0x40..].to_vec(),
                },
            ],
        };

        match adpcm_wii_archive_pool.get_mut(&root_entry.root_path) {
            Some(archive) => archive.1.entries.push(adpcm_wii_entry),
            None => {
                let mut entries = Vec::new();
                entries.push(adpcm_wii_entry);
                adpcm_wii_archive_pool.insert(root_entry.root_path.clone(), (root_entry.root_dir_path, CriAdpcmWiiArchiveWrapper {
                    entries,
                }));
            }
        };
    }

    for entry in adpcm_wii_archive_pool {
        let sound_element_meta_data = fs::read_to_string(entry.1.0.join("sound_element.json"))
            .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data))
            .unwrap()
            .unwrap();
        csb.sound_element.push(CriCsbSoundElement {
            name: entry.0,
            data: CriCsbAdpcmWiiSoundElementData::AdpcmWii(entry.1.1),
            fmt: sound_element_meta_data.fmt,
            nch: sound_element_meta_data.nch,
            stmflg: sound_element_meta_data.stmflg,
            sfreq: sound_element_meta_data.sfreq,
            nsmpl: sound_element_meta_data.nsmpl,
        });

        let cue_meta_info =  fs::read_to_string(entry.1.0.join("cue.json"))
            .map(|data| serde_json::from_str::<CriCsbCue>(&data))
            .unwrap()
            .unwrap();

        csb.cue.push(cue_meta_info);

        let synth_meta_info =  fs::read_to_string(entry.1.0.join("synth.json"))
            .map(|data| serde_json::from_str::<CriCsbSynth>(&data))
            .unwrap()
            .unwrap();

        csb.synth.push(synth_meta_info);

        let isaac_meta_info =  fs::read_to_string(entry.1.0.join("isaac.json"))
            .map(|data| serde_json::from_str::<CriCsbIsaac>(&data))
            .unwrap()
            .unwrap();

        csb.isaac.push(isaac_meta_info);
    }

    let table_info = fs::read_to_string(path.join("table_info.json"))
        .map(|data| serde_json::from_str::<CriCsbInfo>(&data))
        .unwrap()
        .unwrap();

    csb.info.push(table_info);

    let voice_limit_group_info = fs::read_to_string(path.join("table_info.json"))
        .map(|data| serde_json::from_str::<CriCsbVoiceLimitGroup>(&data))
        .unwrap()
        .unwrap();

    csb.voice_limit_group.push(voice_limit_group_info);

    fs::write(format!("{}.csb", path.to_string_lossy()), csb.write().unwrap()).unwrap();
    Ok(())
}