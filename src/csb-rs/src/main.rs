use std::{env, ops, path::Path};

use crate::games::black_knight::{creator, extractor};

mod help;
mod version;

mod games {
    pub mod other {
        pub mod extractor;
        pub mod creator;
    }
    pub mod black_knight {
        pub mod extractor;
        pub mod creator;
    }
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => {
            match &arg[ops::RangeFull] {
                "--help" | "-h" => help::print(),
                "--version" | "-v" => version::print(),
                "extract" => {
                    match args.next() {
                        Some(game) => {
                            match &game[ops::RangeFull] {
                                "black_knight" => {                    
                                    match args.next() {
                                        Some(probably_file) => {
                                            let path = Path::new(&probably_file);
                                            if path.is_file() {
                                                extractor::extract_csb(path);
                                            } else {
                                                panic!("Provided path is not a file");
                                            }
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
                "pack" => {
                    match args.next() {
                        Some(game) => {
                            match &game[ops::RangeFull] {
                                "black_knight" => {                    
                                    match args.next() {
                                        Some(probably_dir) => {
                                            let path = Path::new(&probably_dir);
                                            if path.is_dir() {
                                                creator::create_from_dir(path).unwrap()
                                            } else {
                                                panic!("Provided path is not a directory");
                                            }
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
}