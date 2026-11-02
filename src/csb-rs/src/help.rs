pub fn print() {
    println!("            csb-rs by OSA413
        Built upon Skyth's CsbEditor and SonicAudioLib
        
        Released under the MIT License
        {}

Usage:
    `csb-rs <csb_file_or_dir>`
        Extract the target <csb_file> and corresponding CPK file or re-create if it's a directory

    `csb-rs extract <csb_file>`
        Extract the target <csb_file> that can have a corresponding CPK file

    `csb-rs pack <dir_path>`
        Pack the given directory at <dir_path>

    `csb-rs patch <dir_path>`
        Instead of re-creating the tables from scratch, use <dir_name> and replace only the SOUND_ELEMENT's data elements in <dir_path>.csb (or <dir_path>.cpk) file

    `csb-rs -v` and
    `csb-rs --version`
        Show versions of used tools.

    `csb-rs` and
    `csb-rs -h` and
    `csb-rs --help`
        Show this help message.",
    env!("CARGO_PKG_REPOSITORY"));
}