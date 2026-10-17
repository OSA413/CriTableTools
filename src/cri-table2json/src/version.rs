pub fn print() {
    println!(
        "cri-table2json: {}
cri-table: {}",
        env!("CARGO_PKG_VERSION"),
        cri_table::VERSION,
    );
}