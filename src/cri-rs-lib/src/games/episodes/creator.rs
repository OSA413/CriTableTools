use std::{collections::HashMap, ffi::OsStr, fs, path::{Path, PathBuf}};

use common_binary::error::CommonBinaryError;
use common_utils::walk_dir::walk_dir;
use crate::{aax::{archive::CriAaxArchive, entry::CriAaxEntry, loop_flag::CriAaxLoopFlag}, csb::{common::{cri_csb_aax_archive::CriCsbAaxArchive, cri_csb_aax_sound_element::{CriCsbAaxSoundElement, CriCsbAaxSoundElementData}}, games::{black_knight::archive::CriCsbSoundElementMetaInfo, episodes::archive::{CriCsbAaxCue, CriCsbAaxInfo, CriCsbAaxIsaac, CriCsbAaxSynth, CriCsbAaxVoiceLimitGroup, CriCsbSoundElement}}}, sound_archive_mode::CriSoundArchiveFormat};

static INTRO_ADX_LOWERCASE: &'static str = "intro.adx";
static LOOP_ADX_LOWERCASE: &'static str = "loop.adx";

pub struct ParsedAdx {
    root_path: String,
    root_dir_path: PathBuf,
    meta_info: CriCsbSoundElementMetaInfo,
}

pub fn parse_entry(entry: &Path) -> Option<(ParsedAdx, CriAaxLoopFlag)> {
    let entry_string = entry.to_string_lossy();
    let entry_string_lowercase = entry_string.to_lowercase();
    let (root_dir, loop_flag) = if entry_string_lowercase.ends_with(INTRO_ADX_LOWERCASE) {
        (
            entry_string.chars().take(entry_string.len() - INTRO_ADX_LOWERCASE.len()).collect::<String>(),
            CriAaxLoopFlag::Intro,
        )
    } else if entry_string_lowercase.ends_with(LOOP_ADX_LOWERCASE) {
        (
            entry_string.chars().take(entry_string.len() - LOOP_ADX_LOWERCASE.len()).collect::<String>(),
            CriAaxLoopFlag::Loop,
        )
    } else {
        return None;
    };

    let root_dir_path = Path::new(&root_dir).to_path_buf();
    let meta_info = fs::read_to_string(root_dir_path.join("sound_element.json"))
        .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data).unwrap())
        .unwrap();

    Some((ParsedAdx {
        root_path: root_dir,
        root_dir_path,
        meta_info,
    }, loop_flag))
}

pub fn create_from_dir(path: &Path) -> Result<(), CommonBinaryError> {
    let mut csb = CriCsbAaxArchive {
        info: vec![],
        cue: vec![],
        synth: vec![],
        sound_element: vec![],
        isaac: vec![],
        voice_limit_group: vec![],
    };

    let files = walk_dir(path, None);

    let mut aax_archive_pool: HashMap<String, (PathBuf, CriAaxArchive)> = HashMap::new();

    for entry in files.iter().filter(|entry| entry.to_string_lossy().to_lowercase().ends_with(".adx")) {
        let (root_entry, loop_flag) = match parse_entry(entry) {
            Some(parsed_entry) => parsed_entry,
            None => continue,
        };

        let data = fs::read(entry).unwrap();

        let aax_entry = CriAaxEntry {
            flag: loop_flag,
            data: data,
        };

        match aax_archive_pool.get_mut(&root_entry.root_path) {
            Some(archive) => archive.1.entries.push(aax_entry),
            None => {
                let mut entries = Vec::new();
                entries.push(aax_entry);
                aax_archive_pool.insert(root_entry.root_path.clone(), (root_entry.root_dir_path, CriAaxArchive {
                    name: root_entry.meta_info.name.clone(),
                    entries,
                }));
            }
        };
    }

    for entry in aax_archive_pool {
        let sound_element_meta_data = fs::read_to_string(entry.1.0.join("sound_element.json"))
            .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data))
            .unwrap()
            .unwrap();

        let relative_name = entry.1.0
            .strip_prefix(path).unwrap()
            .to_string_lossy().replace('\\', "/");

        csb.sound_element.push(CriCsbAaxSoundElement {
            format: CriSoundArchiveFormat::Adx,
            name: relative_name,
            data: CriCsbAaxSoundElementData::Aax(entry.1.1),
            fmt: sound_element_meta_data.fmt,
            nch: sound_element_meta_data.nch,
            stmflg: sound_element_meta_data.stmflg,
            sfreq: sound_element_meta_data.sfreq,
            nsmpl: sound_element_meta_data.nsmpl,
        });
    }

    csb.cue = fs::read_to_string(path.join("cue.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbAaxCue>>(&data))
        .unwrap()
        .unwrap();

    csb.synth = fs::read_to_string(path.join("synth.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbAaxSynth>>(&data))
        .unwrap()
        .unwrap();

    csb.info = fs::read_to_string(path.join("table_info.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbAaxInfo>>(&data))
        .unwrap()
        .unwrap();

    let isaac_entries = walk_dir(path, Some(OsStr::new("isaac.json")));

    for entry in isaac_entries {
        let isaac_info = fs::read_to_string(entry)
            .map(|data| serde_json::from_str::<CriCsbAaxIsaac>(&data))
            .unwrap()
            .unwrap();
    
        csb.isaac.push(isaac_info);
    }
    
    csb.voice_limit_group = fs::read_to_string(path.join("voice_limit_group.json"))
        .map(|data| serde_json::from_str::<Vec<CriCsbAaxVoiceLimitGroup>>(&data))
        .unwrap()
        .unwrap();

    fs::write(format!("{}.CSB", path.to_string_lossy()), csb.write().unwrap()).unwrap();
    Ok(())
}