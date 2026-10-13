use common_binary::{binary_reader, endianness::Endianness};

use crate::{field_value::CriFieldValue, table::CriTable};

pub struct CriTableReadingResult {
    pub table: CriTable,
    pub rows_position: usize,
    pub data_pool_position: usize,
    pub row_length: usize,
}

impl CriTableReadingResult {
    pub fn get_field_source(&self, source: &[u8], field_name: &str, row_index: usize) -> Vec<u8> {
        let field_index = self.table.fields.iter().position(|x| x.name == field_name);

        let field_index = match field_index {
            Some(index) => index,
            None => return vec![],
        };

        if field_index >= self.table.fields.len() {
            return vec![];
        }

        let field = &self.table.fields[field_index];
        if !field.flags.row_storage {
            return match &field.rows[row_index] {
                CriFieldValue::Data(data) => data.clone(),
                _ => panic!("Roadside panic")
            }
        }

        let ptr = CriTable::go_to_value_ptr(Some(0), self.rows_position, self.row_length, row_index, field.offset);
        // 0 is header_position in the original C# project, I'm not sure if this will blow up, but so far it works
        let start = 0 + self.data_pool_position + binary_reader::u32::read(source, ptr, &Endianness::Big, "").unwrap() as usize;
        let length = binary_reader::u32::read(source, ptr + 4, &Endianness::Big, "").unwrap() as usize;

        source[start..start + length].to_vec()
    }
}