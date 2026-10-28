use std::{env, ops, path::{Path, PathBuf}, str::FromStr};

use cri_rs_lib::games::{black_knight::{self, creator, extractor}, episodes};

mod help;
mod version;

fn main() {
    let mut command = None;
    let mut game_code = None;
    let mut file_or_dir = None;

    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => {
            match &arg[ops::RangeFull] {
                "--help" | "-h" => help::print(),
                "--version" | "-v" => version::print(),
                "extract" | "pack" => {
                    command = Some(arg);
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
                        None => panic!("Need game name to extract files properly"),
                    }
                },
                probably_path => {
                    println!("Warning, drag&drop API will be deprecated, use CLI for more complex tasks, use `csb-rs -h` to get help.");
                    let path = Path::new(probably_path);
                    if path.is_dir() {
                        creator::create_from_dir(path).unwrap();
                    } else if path.is_file() {
                        extractor::extract_csb(path);
                    }
                }
            }
        }
        None => help::print(),
    }

    if command.is_none()
        || game_code.is_none()
        || file_or_dir.is_none() {
        help::print();
        return;
    }

    let command = command.unwrap();
    let game_code = game_code.unwrap();
    let file_or_dir = file_or_dir.unwrap();

    if command == "extract" {
        if game_code == "episodes" {
            if file_or_dir.is_file() {
                episodes::extractor::extract_csb(&file_or_dir.into_boxed_path());
            } else {
                panic!("Provided path is not a file");
            }
        } else if game_code == "black_knight" {
            if file_or_dir.is_file() {
                black_knight::extractor::extract_csb(&file_or_dir.into_boxed_path());
            } else {
                panic!("Provided path is not a file");
            }
        }
    } else if command == "pack" {
        if game_code == "episodes" {
            if file_or_dir.is_dir() {
                episodes::creator::create_from_dir(&file_or_dir.into_boxed_path()).unwrap();
            } else {
                panic!("Provided path is not a directory");
            }
        } else if game_code == "black_knight" {
            if file_or_dir.is_dir() {
                black_knight::creator::create_from_dir(&file_or_dir.into_boxed_path()).unwrap();
            } else {
                panic!("Provided path is not a directory");
            }
        }
    }
}