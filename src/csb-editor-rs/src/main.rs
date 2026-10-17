use std::{env, ops};

mod help;
mod version;

mod extractor;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => {
            match &arg[ops::RangeFull] {
                "--help" | "-h" => help::print(),
                "--version" | "-v" => version::print(),
                _ => extractor::extract_csb(&arg),
            }
        }
        None => help::print(),
    }
}