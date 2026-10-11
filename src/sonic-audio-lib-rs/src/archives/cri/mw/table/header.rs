use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::archives::cri::mw::table::{field::CriTableField, field_raw_flags::CriFieldRawFlags, field_type::CriFieldType, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType};

pub struct CriTableHeader {
    pub unknown_byte: u8,
    pub encoding_type: CriTableHeaderEncodingType,
    pub rows_position: usize,
    pub data_pool_position: usize,
    pub table_name: String,
    pub row_length: usize,
    pub row_count: usize,
    pub fields: Vec<CriTableField>,
}

impl CriTableHeader {
    pub fn get_field(&self, field_name: &str) -> Option<&CriTableField> {
        self.fields.iter().find(|field| field.name == field_name)
    }

    pub fn go_to_value_ptr(starting_pointer: Option<usize>, rows_position: usize, row_length: usize, row_index: usize, offset: usize) -> usize {
        starting_pointer.unwrap_or(0) + rows_position + (row_length * row_index) + offset
    }

    pub fn read_table(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4]) -> Result<Self, CommonBinaryError> {
        let starting_pointer = pointer.unwrap_or(0);

        if !source[starting_pointer..starting_pointer + 4].eq(&expected_segnature) {
            eprintln!("Invalid signature");
            return Err(CommonBinaryError::SeeConsole());
        }

        let header_unknown_byte = binary_reader::u8::read(source, starting_pointer + 0x08, "")?;
        let header_encoding_type = binary_reader::u8::read(source, starting_pointer + 0x09, "")? as usize;

        let header_rows_position = binary_reader::u16::read(source, starting_pointer + 0x0A, &Endianness::Big, "")? as usize + 0x8;
        let header_string_pool_position = binary_reader::u32::read(source, starting_pointer + 0x0C, &Endianness::Big, "")? as usize + 0x8;
        let header_data_pool_position = binary_reader::u32::read(source, starting_pointer + 0x10, &Endianness::Big, "")? as usize + 0x8;
        let header_relative_table_name_pointer = binary_reader::u32::read(source, starting_pointer + 0x14, &Endianness::Big, "")? as usize;
        let header_field_count = binary_reader::u16::read(source, starting_pointer + 0x18, &Endianness::Big, "")? as usize;
        let header_row_length = binary_reader::u16::read(source, starting_pointer + 0x1A, &Endianness::Big, "")? as usize;
        let header_row_count = binary_reader::u32::read(source, starting_pointer + 0x1C, &Endianness::Big, "")? as usize;

        let pointer = starting_pointer + 0x20;

        let mut fields: Vec<CriTableField> = Vec::with_capacity(header_field_count);

        let mut field_index_pointer = 0;

        let mut row_storage_offset = 0;

        for field_index in 0..header_field_count {
            let field_raw_flags = CriFieldRawFlags::from_bits(binary_reader::u8::read(source, pointer + field_index_pointer, "")?).unwrap();
            let field_flags = field_raw_flags.extract_flags();
            field_index_pointer += 0x01;
            // Maybe redo as a builder pattern?
            let mut field_name_pointer = 0;
            let mut field_position = 0;
            let mut field_length = 0;
            let mut field_offset = 0;
            // /Maybe redo as a builder pattern?

            if field_flags.name
            {
                field_name_pointer = binary_reader::u32::read(source, pointer + field_index_pointer, &Endianness::Big, "")? as usize;
                field_index_pointer += 0x04;
            }

            let field_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position + field_name_pointer, "").unwrap().0;
            let field_type = field_raw_flags.extract_type();

            // This value is shared between all default rows...
            let mut field_default_value = CriFieldValue::Byte(0);

            if field_flags.default_value
            {
                match field_type {
                    CriFieldType::Data => {
                        field_position = binary_reader::u32::read(source, pointer + field_index_pointer, &Endianness::Big, "")? as usize;
                        field_length = binary_reader::u32::read(source, pointer + field_index_pointer + 0x4, &Endianness::Big, "")? as usize;
                        field_index_pointer += 0x08;
                    },
                    _ => {
                        field_default_value = field_type.read_value(source, starting_pointer, header_string_pool_position, header_data_pool_position, pointer + field_index_pointer)?;
                        field_index_pointer += field_type.get_offset();
                    }
                }
            }

            if field_flags.row_storage
            {
                field_offset = row_storage_offset;
                row_storage_offset += field_type.get_offset();
            }

            for row_index in 0..header_row_count {
                let row_value = {
                    if field_flags.row_storage
                    {
                        let value_pointer = CriTableHeader::go_to_value_ptr(Some(starting_pointer), header_rows_position, header_row_length, row_index, field_offset);
                        field_type.read_value(source, starting_pointer, header_string_pool_position, header_data_pool_position, value_pointer)?
                    }
                    else if field_flags.default_value && field_type != CriFieldType::Data
                    {
                        field_default_value.clone()
                    }
                    // Not even per row, and not even constant value? Then there's no storage.
                    else if !field_flags.row_storage && !field_flags.default_value
                    {
                        field_type.clone().get_null_value()
                    } else {
                        CriFieldValue::Byte(0)
                    }
                };

                if fields.len() <= field_index {
                    fields.push(CriTableField {
                        flags: field_flags,
                        ttype: field_type,
                        name: field_name.clone(),
                        source: source[field_position..field_position + field_length].to_vec(),
                        rows: vec![],
                        offset: field_offset,
                    });
                }

                fields[field_index].rows.push(row_value);
            }
        }

        let header_table_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position + header_relative_table_name_pointer as usize, "").unwrap().0;

        Ok(CriTableHeader {
            unknown_byte: header_unknown_byte,
            encoding_type: CriTableHeaderEncodingType::from(header_encoding_type),
            rows_position: header_rows_position,
            data_pool_position: header_data_pool_position,
            table_name: header_table_name,
            row_length: header_row_length,
            row_count: header_row_count,
            fields,
        })
    }

    pub fn get_field_source(&self, source: &[u8], field_name: &str, row_index: usize) -> Vec<u8> {
        let field_index = self.fields.iter().position(|x| x.name == field_name);

        let field_index = match field_index {
            Some(index) => index,
            None => return vec![],
        };

        if field_index >= self.fields.len() {
            return vec![];
        }

        let field = &self.fields[field_index];
        if !field.flags.row_storage {
            return field.source.clone();
        }

        let ptr = CriTableHeader::go_to_value_ptr(Some(0), self.rows_position, self.row_length, row_index, field.offset);
        // 0 is header_position in the original C# project, I'm not sure if this will blow up, but so far it works
        let start = 0 + self.data_pool_position + binary_reader::u32::read(source, ptr, &Endianness::Big, "").unwrap() as usize;
        let length = binary_reader::u32::read(source, ptr + 4, &Endianness::Big, "").unwrap() as usize;

        return source[start..start + length].to_vec();
    }
}