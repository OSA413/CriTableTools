#[derive(Debug)]
pub struct TableStringPoolBuilderEntry {
    pub string: String,
    pub pointer: u32,
}

pub struct TableDataPoolBuilderEntry {
    // Maybe box?
    pub data: Vec<u8>,
    pub hash: String,
    pub pointer: usize,
}