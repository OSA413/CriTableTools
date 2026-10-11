pub const UTF_SIGNATURE: [u8; 4] = *b"@UTF";

pub fn is_table_data(source: &[u8], pointer: Option<usize>) -> bool {
    let pointer = pointer.unwrap_or(0);
    source.len() - pointer >= UTF_SIGNATURE.len() && source[pointer..pointer+UTF_SIGNATURE.len()] == UTF_SIGNATURE
}
