pub fn print() {
    println!(
        "csb-editor-rs: {}
cri-table: {}",
        env!("CARGO_PKG_VERSION"),
        cri_table::VERSION,
    );
}