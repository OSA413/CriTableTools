use std::{env, fs, ops, path::{Path, PathBuf}, str::FromStr};

use common_utils::walk_dir::walk_dir;
use cri_rs_lib::games::{black_knight, episodes};
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
            // TODO figure out the order of the elements arrays to not matter
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
        let original_file = &extracted_files.iter().find(|file| file.strip_prefix(extracted_dir_path).unwrap() == relative_file_to_check).unwrap_or_else(|| panic!("Corresponging recreated file for {} not found", file_to_check.to_string_lossy()));

        if let Some(extension) = file_to_check.extension()
        && extension == "json" {
            // TODO figure out the order of the elements arrays to not matter
        } else {
            assert_eq!(
                get_sha256(fs::read(&file_to_check).unwrap()),
                get_sha256(fs::read(&original_file).unwrap()),
                "Files {} and {} are expected to be equal", file_to_check.to_string_lossy(), original_file.to_string_lossy()
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