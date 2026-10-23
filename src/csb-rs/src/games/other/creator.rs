use std::{collections::HashMap, fs, path::Path};

use common_binary::error::CommonBinaryError;
use common_utils::walk_dir::walk_dir;
use cri_rs_lib::{aax::{archive::CriAaxArchive, entry::CriAaxEntry, loop_flag::CriAaxLoopFlag}, csb::games::black_knight::archive::CriCsbArchive};

static INTRO_ADX: &'static str = "intro.adx";
static LOOP_ADX: &'static str = "loop.adx";

pub fn parse_entry(entry: &Path) -> Option<(String, CriAaxLoopFlag)> {
    let entry_string = entry.to_string_lossy();
    let entry_string_lowercase = entry_string.to_lowercase();
    if entry_string_lowercase.ends_with(INTRO_ADX) {
        return Some((
            entry_string.chars().take(entry_string.len() - INTRO_ADX.len()).collect::<String>(),
            CriAaxLoopFlag::Intro,
        ))
    } else if entry_string_lowercase.ends_with("loop.adx") {
        return Some((
            entry_string.chars().take(entry_string.len() - LOOP_ADX.len()).collect::<String>(),
            CriAaxLoopFlag::Loop,
        ))
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

    let mut aax_archive_pool: HashMap<String, CriAaxArchive> = HashMap::new();

    for entry in files.iter().filter(|entry| entry.ends_with(".adx") || entry.ends_with(".ADX")) {
        let Some((
            root_entry,
            aax_flag
        )) = parse_entry(entry) else { continue };

        let aax_entry = CriAaxEntry {
            flag: aax_flag,
            data: fs::read(&entry).unwrap(),
        };

        // match aax_archive_pool.get_mut(&root_entry) {
        //     Some(archive) => archive.entries.push(aax_entry),
        //     None => {
        //         let mut entries = Vec::with_capacity(2);
        //         entries.push(aax_entry);
        //         aax_archive_pool.insert(root_entry.clone(), CriAaxArchive {
        //             name: 
        //             entries,
        //         });
        //     }
        // };
    }

    // for entry in aax_archive_pool {
    //     csb.sound_element.push(CriCsbSoundElement {
    //         name: entry.0,
    //         CriCsbSoundElement::Aax(entry.1)
    //     });
    // }

    fs::write(format!("{}.csb", path.to_string_lossy()), csb.write().unwrap()).unwrap();
    Ok(())
}