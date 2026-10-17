pub fn print() {
    println!("            cri-table2json by OSA413
        Built upon Skyth's SonicAudioLib
        
        Released under the MIT License
        {}

Usage:
    `cri-table2json <file>`
        Print the content of <file> if it's a CriTable file (CSB, CPK)
    
    `cri-table2json -v` and
    `cri-table2json --version`
        Show versions of used tools.

    `cri-table2json` and
    `cri-table2json -h` and
    `cri-table2json --help`
        Show this help message.",
    env!("CARGO_PKG_REPOSITORY"));
}