pub fn print() {
    println!("            csb-rs by OSA413
        Built upon Skyth's CsbEditor and SonicAudioLib
        
        Released under the MIT License
        {}

Usage:
    `csb-rs <csb_file>`
        Extract the target <csb_file> and corresponding CPK file

    `csb-rs extract <game_code> <csb_file>`
        Extract the target <csb_file> from a game under the <game_code> that can have a corresponding CPK file
    
    `csb-rs -v` and
    `csb-rs --version`
        Show versions of used tools.

    `csb-rs` and
    `csb-rs -h` and
    `csb-rs --help`
        Show this help message.",
    env!("CARGO_PKG_REPOSITORY"));
}