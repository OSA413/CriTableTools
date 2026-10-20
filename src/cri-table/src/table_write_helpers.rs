pub struct TableHeaderWriteResult {
    pub data: Vec<u8>,
    pub pointer: usize,
    pub length_pointer: usize,
}

pub struct TableHeaderInfoWriteResult {
    pub data: Vec<u8>,
    pub pointer: usize,
    pub row_position_pointer: usize,
    pub string_pool_position_pointer: usize,
    pub data_pool_position_pointer: usize,
    pub table_name_position_pointer: usize,
}

pub struct TableFieldsInfoWriteResult {
    pub data: Vec<u8>,
    pub pointer: usize,
    pub length: usize,
}

pub struct TableStringPoolBUilderEntry {
    pub string: String,
    pub pointer: u32,
}