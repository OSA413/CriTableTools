use std::{env, fs::{self, File}, io::{BufRead, BufReader, Read}, ops, path::{Path, PathBuf}, str::FromStr};

use cri_rs_lib::{find_related_csb_of_dir, games::{black_knight::{self, creator, extractor}, episodes}, sound_archive_mode::CriSoundArchiveFormat};
use cri_table::table::UTF_SIGNATURE;

mod help;
mod version;

fn main() {
    // TODO redo as enums instead of strings
    let mut cli_command = None;
    let mut cli_game_code = None;
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
                        Some(game) => {
                            match &game[ops::RangeFull] {
                                "black_knight" | "episodes" => {
                                    cli_game_code = Some(game);
                                    match args.next() {
                                        Some(probably_file) => {
                                            cli_file_or_dir = Some(PathBuf::from_str(&probably_file).unwrap());
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
                    let file_or_dir = Path::new(probably_path);
                    cli_file_or_dir = Some(file_or_dir.to_path_buf());
                    let probably_format_file = file_or_dir.join("format.json");
                    if file_or_dir.is_dir() {
                        let original_csb_file = find_related_csb_of_dir(file_or_dir);
                        if let Some(original_csb_file) = original_csb_file {
                            if original_csb_file.is_file() {
                                cli_command = Some("patch".to_string());
                            } else {
                                panic!("Something's wrong. For the provided directory a thing with CSB extension is found, but it's not a file!");
                            }
                        } else {
                            match fs::read_to_string(probably_format_file) {
                                Ok(content) => {
                                    match serde_json::from_str::<CriSoundArchiveFormat>(&content) {
                                        Ok(format) => {
                                            match format {
                                                CriSoundArchiveFormat::Adx => cli_game_code = Some("episodes".to_string()),
                                                CriSoundArchiveFormat::Dsp => cli_game_code = Some("black_knight".to_string()),
                                                CriSoundArchiveFormat::Wav => todo!("Wav container is not supported."),
                                            }
                                        },
                                        Err(e) => panic!("Provided directory doesn't have the \"format.json\" file with the wrong JSON structure or it's not with the proper structure or values to properly create a CSB file from the directory.
If you wanted to patch the file, please make sure that the original CSB file is located in the same directory as the directory you provided.
The error is: {e}"),
                                    }
                                },
                                Err(e) => panic!("Provided directory doesn't have the \"format.json\" or it's not with the proper structure and/or values to properly create a CSB file from the directory.
If you wanted to patch the file, please make sure that the original CSB file is located in the same directory as the directory you provided.
The error is: {e}"),
                            }
                            cli_command = Some("create".to_string());
                        }
                    } else if file_or_dir.is_file() {
                        {
                            let f = File::open("file_or_dir").unwrap();
                            let reader = BufReader::new(f);
                            let reader_at_signature = reader.take(0x4);
                            if reader_at_signature.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>() != UTF_SIGNATURE {
                                panic!("The provided file doesn't have the @UTF signature at the beginning, are you sure it's a CRI table?");
                            }
                        }

                        {
                            let f = File::open("file_or_dir").unwrap();
                            let mut reader = BufReader::new(f);
                            while reader.skip_until(b'A').is_ok().to_owned() {
                                let mut buf: Vec<u8> = vec![];
                                if reader.read_until(0, &mut buf).is_ok() {
                                    if buf == b"AAX" {
                                        cli_game_code = Some("episodes".to_string());
                                        break;
                                    } else if buf == b"ADPCM_WII" {
                                        cli_game_code = Some("black_knight".to_string());
                                        break;
                                    }
                                }
                            }
                        }

                        if cli_game_code.is_none() {
                            panic!("Couldn't detect the archive type from the file to automatically set the <game_code> for extraction. Please use CLI. Use `csb-rs --help` to get help.");
                        }
                    }
                }
            }
        }
        None => help::print(),
    }

    if cli_command.is_none()
        || cli_game_code.is_none()
        || cli_file_or_dir.is_none() {
        println!("Not all of the parameters were successfully parsed.");
        if cli_command.is_none() {
            println!("<command> was not parsed");
        }
        if cli_game_code.is_none() {
            println!("<game_code> was not parsed");
        }
        if cli_file_or_dir.is_none() {
            println!("<file_or_dir> was not parsed.");
        }
        help::print();
        return;
    }

    let command = cli_command.unwrap();
    let game_code = cli_game_code.unwrap();
    let file_or_dir = cli_file_or_dir.unwrap();

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
    } else if command == "patch" {
        if game_code == "episodes" {
            // if file_or_dir.is_dir() {
            //     episodes::creator::create_from_dir(&file_or_dir.into_boxed_path()).unwrap();
            // } else {
            //     panic!("Provided path is not a directory");
            // }
        } else if game_code == "black_knight" {
            if file_or_dir.is_file() {
                black_knight::patcher::patch(&file_or_dir.into_boxed_path());
            } else {
                panic!("Provided path is not a directory");
            }
        }
    }
}