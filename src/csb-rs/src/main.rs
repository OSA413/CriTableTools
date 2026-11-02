use std::{env, ops, path::{Path, PathBuf}, str::FromStr};

use cri_rs_lib::{csb, find_related_csb_of_dir};

mod help;
mod version;

fn main() {
    // TODO redo as enums instead of strings
    let mut cli_command = None;
    let mut cli_file_or_dir = None;

    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => {
            match &arg[ops::RangeFull] {
                "--help" | "-h" => help::print(),
                "--version" | "-v" => version::print(),
                "extract" | "pack" | "patch" => {
                    cli_command = Some(arg);
                    match args.next() {
                        Some(probably_file) => {
                            cli_file_or_dir = Some(PathBuf::from_str(&probably_file).unwrap());
                        },
                        None => panic!("Need file path"),
                    }
                },
                probably_path => {
                    let file_or_dir = Path::new(probably_path);
                    cli_file_or_dir = Some(file_or_dir.to_path_buf());
                    if file_or_dir.is_dir() {
                        let original_csb_file = find_related_csb_of_dir(file_or_dir);
                        if let Some(original_csb_file) = original_csb_file {
                            if original_csb_file.is_file() {
                                cli_command = Some("patch".to_string());
                            } else {
                                panic!("Something's wrong. For the provided directory a thing with CSB extension is found, but it's not a file!");
                            }
                        } else {
                            cli_command = Some("create".to_string());
                        }
                    } else if file_or_dir.is_file() {
                        cli_command = Some("extract".to_string());
                    }
                }
            }
        }
        None => help::print(),
    }

    if cli_command.is_none()
        || cli_file_or_dir.is_none() {
        println!("Not all of the parameters were successfully parsed.");
        if cli_command.is_none() {
            println!("<command> was not parsed");
        }
        if cli_file_or_dir.is_none() {
            println!("<file_or_dir> was not parsed.");
        }
        help::print();
        return;
    }

    let command = cli_command.unwrap();
    let file_or_dir = cli_file_or_dir.unwrap();

    if command == "extract" {
        if file_or_dir.is_file() {
            csb::extractor::extract(&file_or_dir.into_boxed_path());
        }
    } else if command == "pack" {
        if file_or_dir.is_dir() {
            csb::creator::create_from_dir(&file_or_dir.into_boxed_path()).unwrap();
        } else {
            panic!("Provided path is not a directory");
        }
    } else if command == "patch" {
        if file_or_dir.is_file() {
            csb::patcher::patch(&file_or_dir.into_boxed_path());
        } else {
            panic!("Provided path is not a directory");
        }
    }
}