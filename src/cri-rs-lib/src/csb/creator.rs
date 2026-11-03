use std::{ffi::OsStr, fs, path::{Path, PathBuf}};

use common_binary::error::CommonBinaryError;
use common_utils::walk_dir::walk_dir;
use crate::{adpcm_wii::entry::CriAdpcmWiiEntry, csb::{common::{archive::CriCsbArchive, cue::CriCsbCue, info::CriCsbInfo, isaac::CriCsbIsaac, synth::CriCsbSynth, voice_limit_group::CriCsbVoiceLimitGroup}, sound_element::{CriCsbSoundElement, sound_element_metainfo::CriCsbSoundElementMetaInfo}}, loop_flag::CriLoopFlag, sound_archive::CriCsbSoundArchiveEnum, sound_archive_mode::CriSoundArchiveFormat, traits::ToCriTable};

static INTRO_LOWERCASE: &str = "intro";
static LOOP_LOWERCASE: &str = "loop";

pub struct ParsedMatadata {
    pub root_path: String,
    pub root_dir_path: PathBuf,
    pub meta_info: CriCsbSoundElementMetaInfo,
}

pub fn parse_entry(entry: &Path, expected_format: &CriSoundArchiveFormat) -> Option<(ParsedMatadata, CriLoopFlag)> {
    let entry_string = entry.to_string_lossy();
    let entry_string_lowercase = entry_string.to_lowercase();

    let intro_name_ending = Path::new(INTRO_LOWERCASE).with_added_extension(match expected_format {
        CriSoundArchiveFormat::Adx => "adx",
        CriSoundArchiveFormat::Dsp => "dsp",
        CriSoundArchiveFormat::Wav => "wav",
    }).to_string_lossy().into_owned();
    
    let loop_name_ending = Path::new(LOOP_LOWERCASE).with_added_extension(match expected_format {
        CriSoundArchiveFormat::Adx => "adx",
        CriSoundArchiveFormat::Dsp => "dsp",
        CriSoundArchiveFormat::Wav => "wav",
    }).to_string_lossy().into_owned();

    let (root_dir, loop_flag) = if entry_string_lowercase.ends_with(&intro_name_ending) {
        (
            entry_string.chars().take(entry_string.len() - intro_name_ending.len()).collect::<String>(),
            CriLoopFlag::Intro,
        )
    }
    else if entry_string_lowercase.ends_with(&loop_name_ending) {
        (
            entry_string.chars().take(entry_string.len() - loop_name_ending.len()).collect::<String>(),
            CriLoopFlag::Loop,
        )
    } else {
        return None
    };
    let root_dir_path = Path::new(&root_dir).to_path_buf();

    let meta_info = fs::read_to_string(root_dir_path.join("sound_element.json"))
        .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data).unwrap())
        .unwrap();

    let parsed_entry = ParsedMatadata {
        root_path: root_dir,
        root_dir_path,
        meta_info,
    };

    Some((parsed_entry, loop_flag))
}

pub fn create_from_dir(path: &Path) -> Result<(), CommonBinaryError> {
    let mut csb = CriCsbArchive {
        original_string_pool: vec![],
        info: (vec![],vec![]),
        cue: (vec![],vec![]),
        synth: (vec![],vec![]),
        sound_element: (vec![],vec![]),
        isaac: (vec![],vec![]),
        voice_limit_group: (vec![],vec![]),
    };

    let files = walk_dir(path, None);
    let adx_files = files.iter().filter(|x | x.to_string_lossy().to_lowercase().ends_with(".adx")).count();
    let dsp_files = files.iter().filter(|x | x.to_string_lossy().to_lowercase().ends_with(".dsp")).count();
    let expected_format = if adx_files >= dsp_files {
        CriSoundArchiveFormat::Adx
    } else {
        CriSoundArchiveFormat::Dsp
    };

    let files = files.iter().filter(|x| match &expected_format {
        CriSoundArchiveFormat::Adx => x.to_string_lossy().ends_with(".adx"),
        CriSoundArchiveFormat::Dsp => x.to_string_lossy().ends_with(".dsp"),
        CriSoundArchiveFormat::Wav => x.to_string_lossy().ends_with(".wav"),
    });

    for entry in files {
        let (root_entry, loop_flag) = match parse_entry(entry, &expected_format) {
            Some(parsed_entry) => parsed_entry,
            None => continue,
        };

        let data = fs::read(entry).unwrap();
        let relative_name = root_entry.root_dir_path
            .strip_prefix(path).unwrap()
            .to_string_lossy().replace('\\', "/");

        let data = match expected_format {
            CriSoundArchiveFormat::Adx => todo!(),
            CriSoundArchiveFormat::Dsp => CriCsbSoundArchiveEnum::AdpcmWii(
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
            ),
            CriSoundArchiveFormat::Wav => todo!(),
        };

        let sound_element_meta_data = fs::read_to_string(entry.join("sound_element.json"))
            .map(|data| serde_json::from_str::<CriCsbSoundElementMetaInfo>(&data))
            .unwrap()
            .unwrap();

        csb.sound_element.0.push(CriCsbSoundElement {
            original_string_pool: vec![],
            name: relative_name.clone(),
            data: data,
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

    fs::write(path.with_added_extension("csb"), csb.to_bytes().unwrap()).unwrap();

    Ok(())
}