use std::{collections::HashMap, ffi::OsStr, fs, path::{Path, PathBuf}};

use common_binary::error::CommonBinaryError;
use common_utils::walk_dir::walk_dir;
use crate::{adpcm_wii::{archive::{CriAdpcmWiiArchive, CriAdpcmWiiArchiveWrapper}, entry::CriAdpcmWiiEntry}, csb::common::{adpcm_wii_archive::CriCsbAdpcmWiiArchive, adpcm_wii_sound_element::CriCsbAdpcmWiiSoundElement, cue::CriCsbCue, info::CriCsbInfo, isaac::CriCsbIsaac, sound_element_metainfo::CriCsbSoundElementMetaInfo, synth::CriCsbSynth, voice_limit_group::CriCsbVoiceLimitGroup}, loop_flag::CriLoopFlag, sound_archive_mode::CriSoundArchiveFormat};

static INTRO_DSP_LOWERCASE: &str = "intro.dsp";
static LOOP_DSP_LOWERCASE: &str = "loop.dsp";

pub struct ParsedDsp {
    pub root_path: String,
    pub root_dir_path: PathBuf,
    pub meta_info: CriCsbSoundElementMetaInfo,
}

pub fn parse_entry(entry: &Path) -> Option<(ParsedDsp, CriLoopFlag)> {
    let entry_string = entry.to_string_lossy();
    let entry_string_lowercase = entry_string.to_lowercase();

    let (root_dir, loop_flag) = if entry_string_lowercase.ends_with(INTRO_DSP_LOWERCASE) {
        (
            entry_string.chars().take(entry_string.len() - INTRO_DSP_LOWERCASE.len()).collect::<String>(),
            CriLoopFlag::Intro,
        )
    }
    else if entry_string_lowercase.ends_with(LOOP_DSP_LOWERCASE) {
        (
            entry_string.chars().take(entry_string.len() - LOOP_DSP_LOWERCASE.len()).collect::<String>(),
            CriLoopFlag::Loop,
        )
    } else {
        return None
    };
    let root_dir_path = Path::new(&root_dir).to_path_buf();

    let meta_info = fs::read_to_string(root_dir_path.join("sound_element.json"))
        .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data).unwrap())
        .unwrap();

    Some((ParsedDsp {
        root_path: root_dir,
        root_dir_path,
        meta_info,
    }, loop_flag))
}

pub fn create_from_dir(path: &Path) -> Result<(), CommonBinaryError> {
    let mut csb = CriCsbAdpcmWiiArchive {
        original_string_pool: vec![],
        info: (vec![],vec![]),
        cue: (vec![],vec![]),
        synth: (vec![],vec![]),
        sound_element: (vec![],vec![]),
        isaac: (vec![],vec![]),
        voice_limit_group: (vec![],vec![]),
    };

    let files = walk_dir(path, None);

    let mut adpcm_wii_archive_pool: HashMap<String, (PathBuf, CriAdpcmWiiArchiveWrapper)> = HashMap::new();

    for entry in files.iter().filter(|entry| entry.to_string_lossy().to_lowercase().ends_with(".dsp")) {
        let (root_entry, loop_flag) = match parse_entry(entry) {
            Some(parsed_entry) => parsed_entry,
            None => continue,
        };

        let data = fs::read(entry).unwrap();

        let adpcm_wii_entry = CriAdpcmWiiArchive {
            format: CriSoundArchiveFormat::Dsp,
            original_string_pool: root_entry.meta_info.original_string_pool.clone(),
            fmt: root_entry.meta_info.fmt,
            name: root_entry.meta_info.name.clone(),
            nch: root_entry.meta_info.nch,
            nsmpl: root_entry.meta_info.nsmpl,
            sfreq: root_entry.meta_info.sfreq,
            stmflg: root_entry.meta_info.stmflg,
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
        };

        match adpcm_wii_archive_pool.get_mut(&root_entry.root_path) {
            Some(archive) => archive.1.entries.push(adpcm_wii_entry),
            None => {
                let mut entries = Vec::new();
                entries.push(adpcm_wii_entry);
                adpcm_wii_archive_pool.insert(root_entry.root_path.clone(), (root_entry.root_dir_path, CriAdpcmWiiArchiveWrapper {
                    original_string_pool: root_entry.meta_info.original_string_pool,
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

        let relative_name = entry.1.0
            .strip_prefix(path).unwrap()
            .to_string_lossy().replace('\\', "/");

        csb.sound_element.0.push(CriCsbAdpcmWiiSoundElement {
            // format: CriSoundArchiveFormat::Adx,
            original_string_pool: vec![],
            name: relative_name,
            data: entry.1.1,
            fmt: sound_element_meta_data.fmt,
            nch: sound_element_meta_data.nch,
            stmflg: sound_element_meta_data.stmflg,
            sfreq: sound_element_meta_data.sfreq,
            nsmpl: sound_element_meta_data.nsmpl,
        });
    }

    csb.cue.0 = fs::read_to_string(path.join("cue.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbCue>>(&data))
        .unwrap()
        .unwrap();

    csb.synth.0 = fs::read_to_string(path.join("synth.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbSynth>>(&data))
        .unwrap()
        .unwrap();

    csb.info.0 = fs::read_to_string(path.join("table_info.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbInfo>>(&data))
        .unwrap()
        .unwrap();

    let isaac_entries = walk_dir(path, Some(OsStr::new("isaac.json")));

    for entry in isaac_entries {
        let isaac_info = fs::read_to_string(entry)
            .map(|data| serde_json::from_str::<CriCsbIsaac>(&data))
            .unwrap()
            .unwrap();
    
        csb.isaac.0.push(isaac_info);
    }
    
    csb.voice_limit_group.0 = fs::read_to_string(path.join("voice_limit_group.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbVoiceLimitGroup>>(&data))
        .unwrap()
        .unwrap();

    fs::write(path.with_added_extension("CSB"), csb.write().unwrap()).unwrap();

    Ok(())
}