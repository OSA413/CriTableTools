pub fn print() {
    println!("            csb-editor-rs by OSA413
        Built upon Skyth's CsbEditor and SonicAudioLib
        
        Released under the MIT License
        {}

Usage:
    `csb-editor-rs <csb_file>`
        Extract the target <csb_file> and corresponding CPK file
    
    `csb-editor-rs -v` and
    `csb-editor-rs --version`
        Show versions of used tools.

    `csb-editor-rs` and
    `csb-editor-rs -h` and
    `csb-editor-rs --help`
        Show this help message.",
    env!("CARGO_PKG_REPOSITORY"));
}