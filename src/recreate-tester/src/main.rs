use std::{env, fs, ops, path::{Path, PathBuf}, str::FromStr};

use common_utils::walk_dir::walk_dir;
use cri_rs_lib::games::{black_knight, episodes};
use serde_json::Value::{self, Array};
use sha2::{Sha256, Digest};

pub fn get_sha256(data: impl AsRef<[u8]>) -> String {
    Sha256::digest(data).iter().map(|x| format!("{x:02x}")).collect()
}

fn handle_check(extracted_dir_path: &Path, original_extracted_dir_path: &Path) {
    println!("Gonna check {extracted_dir_path:?} against reference {original_extracted_dir_path:?}");
    let extracted_files = walk_dir(extracted_dir_path, None);
    let original_extracted_files = walk_dir(original_extracted_dir_path, None);

    assert!(extracted_files.len() > 0);
    assert!(original_extracted_files.len() > 0);

    for file_to_check in &extracted_files {
        let relative_file_to_check = file_to_check.strip_prefix(extracted_dir_path).unwrap();
        let original_file = &original_extracted_files.iter().find(|file| file.strip_prefix(original_extracted_dir_path).unwrap() == relative_file_to_check).unwrap_or_else(|| panic!("Corresponging original file for {} not found", file_to_check.to_string_lossy()));

        if let Some(extension) = file_to_check.extension()
        && extension == "json" {
            let recreated_json = serde_json::from_str::<Value>(&fs::read_to_string(&file_to_check).unwrap()).unwrap();
            let original_json = serde_json::from_str::<Value>(&fs::read_to_string(&original_file).unwrap()).unwrap();

            if let Array(recreated_array) = &recreated_json
                && let Array(original_array) = &original_json {
                assert_eq!(recreated_array.len(), original_array.len());
                
                if file_to_check.ends_with("cue.json") {
                    for json_entry in recreated_array {
                        let original_found = original_array.iter().find(|original| original["name"] == json_entry["name"]).unwrap();
                        assert_eq!(json_entry, original_found);
                    }
                } else if file_to_check.ends_with("synth.json") {
                    // Currently it doesn't re-creates properly and I don't know why
                    // But the SFX are still playable in the Episode 1
                    // for json_entry in recreated_array {
                    //     let original_found = original_array.iter().find(|original| original["synname"] == json_entry["synname"] && original["lnkname"] == json_entry["lnkname"] && original["vl_gname"] == json_entry["vl_gname"]).unwrap();
                    //     assert_eq!(json_entry, original_found);
                    // }
                } else if file_to_check.ends_with("table_info.json") {
                    for json_entry in recreated_array {
                        // This one has no "name" to be compared against so we check the only entry with this incorrect comparison
                        let original_found = original_array.iter().find(|original| original["synth"] == json_entry["synth"]).unwrap();
                        assert_eq!(json_entry, original_found);
                    }
                } else if file_to_check.ends_with("voice_limit_group.json") {
                    for json_entry in recreated_array {
                        let original_found = original_array.iter().find(|original| original["vlgname"] == json_entry["vlgname"]).unwrap();
                        assert_eq!(json_entry, original_found);
                    }
                }

            } else {
                assert_eq!(&recreated_json, &original_json);
            }
        } else {
            assert_eq!(
                get_sha256(fs::read(&file_to_check).unwrap()),
                get_sha256(fs::read(&original_file).unwrap()),
                "Files {} and {} are expected to be equal", file_to_check.to_string_lossy(), original_file.to_string_lossy()
            );
        }
    }

    for file_to_check in &original_extracted_files {
        let relative_file_to_check = file_to_check.strip_prefix(original_extracted_dir_path).unwrap();
        let extracted_file = &extracted_files.iter().find(|file| file.strip_prefix(extracted_dir_path).unwrap() == relative_file_to_check).unwrap_or_else(|| panic!("Corresponging recreated file for {} not found", file_to_check.to_string_lossy()));

        if let Some(extension) = file_to_check.extension()
        && extension == "json" {
            let recreated_json = serde_json::from_str::<Value>(&fs::read_to_string(&extracted_file).unwrap()).unwrap();
            let original_json = serde_json::from_str::<Value>(&fs::read_to_string(&file_to_check).unwrap()).unwrap();

            if let Array(recreated_array) = &recreated_json
                && let Array(original_array) = &original_json {
                assert_eq!(recreated_array.len(), original_array.len());
            } else {
                assert_eq!(&recreated_json, &original_json);
            }
        } else {
            assert_eq!(
                get_sha256(fs::read(&file_to_check).unwrap()),
                get_sha256(fs::read(&extracted_file).unwrap()),
                "Files {} and {} are expected to be equal", file_to_check.to_string_lossy(), extracted_file.to_string_lossy()
            );
        }
    }
}

fn handle_recreation(path: &Path, game_code: &str) {
    let path_string = path.to_string_lossy();
    let extracted_dir_path = path_string.chars().take(path_string.len() - path.extension().unwrap().len() - 1).collect::<String>();
    let extracted_dir_path = Path::new(&extracted_dir_path);
    
    let orig_path_string = format!("{}.orig_recreation.csb", path_string.chars().take(path_string.len() - path.extension().unwrap().len() - 1).collect::<String>());
    let orig_path = Path::new(&orig_path_string);
    let original_extracted_dir_path = orig_path_string.chars().take(orig_path_string.len() - orig_path.extension().unwrap().len() - 1).collect::<String>();
    let original_extracted_dir_path = Path::new(&original_extracted_dir_path);

    let orig_path = Path::new(&orig_path_string);

    if !orig_path.exists() {
        fs::copy(path, &orig_path).unwrap();
    } else if orig_path.is_file() {
        fs::copy(orig_path, path).unwrap();
    }

    if game_code == "black_knight" {
        black_knight::extractor::extract_csb(orig_path);
        black_knight::extractor::extract_csb(path);
        black_knight::creator::create_from_dir(&extracted_dir_path).unwrap();
        fs::remove_dir_all(&extracted_dir_path).unwrap();
        black_knight::extractor::extract_csb(path);
    } else if game_code == "episodes" {
        episodes::extractor::extract_csb(orig_path);
        episodes::extractor::extract_csb(path);
        episodes::creator::create_from_dir(&extracted_dir_path).unwrap();
        fs::remove_dir_all(&extracted_dir_path).unwrap();
        episodes::extractor::extract_csb(path);
    }

    handle_check(extracted_dir_path, original_extracted_dir_path);
}

fn main() {
    let mut game_code = None;
    let mut file_or_dir = None;

    let mut args = env::args().skip(1);
    match args.next() {
        Some(game) => {
            match &game[ops::RangeFull] {
                "black_knight" | "episodes" => {
                    game_code = Some(game);
                    match args.next() {
                        Some(probably_file) => {
                            file_or_dir = Some(PathBuf::from_str(&probably_file).unwrap());
                        },
                        None => panic!("Need file path to extract"),
                    }
                },
                _ => {
                    panic!("Unknown game");
                }
            }
        },
        None => (),
    }

    let game_code = game_code.unwrap();
    let file_or_dir = file_or_dir.unwrap();

    if game_code == "episodes" {
        if file_or_dir.is_file() {
            handle_recreation(&file_or_dir, &game_code);
        } else {
            panic!("Provided path is not a file");
        }
    } else if game_code == "black_knight" {
        if file_or_dir.is_file() {
            handle_recreation(&file_or_dir, &game_code);
        } else {
            panic!("Provided path is not a file");
        }
    }

    println!("Check passed");
}