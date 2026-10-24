pub fn print() {
    println!("            csb-rs by OSA413
        Built upon Skyth's CsbEditor and SonicAudioLib
        
        Released under the MIT License
        {}

Supported <game_code>s are: \"black_knight\", \"episodes\".
If your game is not supported, you can try your luck with the supported game codes.
If it still doesn't work, then try opening an issue (please don't share proprietary files).

Usage:
    `csb-rs <csb_file>`
        Extract the target <csb_file> and corresponding CPK file

    `csb-rs extract <game_code> <csb_file>`
        Extract the target <csb_file> from a game under the <game_code> that can have a corresponding CPK file

    `csb-rs pack <game_code> <dir_path>`
        Pack the given directory at <dir_path> for a game under the <game_code>

    `csb-rs pack <game_code> <dir_path>`
        Instead of re-creating the tables from scratch, use <dir_name> and replace only the SOUND_ELEMENT's data elements in <dir_path>.csb (or <dir_path>.cpk) file for a game under the <game_code>

    `csb-rs -v` and
    `csb-rs --version`
        Show versions of used tools.

    `csb-rs` and
    `csb-rs -h` and
    `csb-rs --help`
        Show this help message.",
    env!("CARGO_PKG_REPOSITORY"));
}