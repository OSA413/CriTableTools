mod dump;
mod help;
mod version;

use std::{env, ops};

use crate::dump::dump_file;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => {
            match &arg[ops::RangeFull] {
                "--help" | "-h" => help::print(),
                "--version" | "-v" => version::print(),
                _ => dump_file(&arg),
            }
        }
        None => help::print(),
    }
}
