use common_binary::{binary_reader, binary_writer, endianness::Endianness, error::CommonBinaryError};

use crate::{field::CriTableField, field_raw_flags::CriFieldRawFlags, field_type::CriFieldType, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType};

pub const UTF_SIGNATURE: [u8; 4] = *b"@UTF";

#[derive(Clone, Debug, PartialEq)]
pub struct CriTable {
    pub name: String,
    pub encoding: CriTableHeaderEncodingType,
    pub fields: Vec<CriTableField>,
    pub unknown_byte: u8,
}

impl CriTable {
    pub fn row_count(&self) -> usize {
        match self.fields.first() {
            Some(field) => field.rows.len(),
            None => 0,
        }
    }

    pub fn get_field(&self, field_name: &str) -> Option<&CriTableField> {
        self.fields.iter().find(|field| field.name == field_name)
    }

    pub fn is_table(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4]) -> bool {
        let starting_pointer = pointer.unwrap_or(0);
        source[starting_pointer..starting_pointer + 4].eq(&expected_segnature)
    }

    pub fn read(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4]) -> Result<CriTable, CommonBinaryError> {
        let starting_pointer = pointer.unwrap_or(0);

        if !CriTable::is_table(source, pointer, expected_segnature) {
            eprintln!("Invalid signature, expected {expected_segnature:?}, got {:?} at {starting_pointer}", source[starting_pointer..starting_pointer + 4].to_vec());
            return Err(CommonBinaryError::SeeConsole());
        }

        let header_unknown_byte = binary_reader::u8::read(source, starting_pointer + 0x08, "")?;
        let header_encoding_type = binary_reader::u8::read(source, starting_pointer + 0x09, "")? as usize;

        let header_rows_position = binary_reader::u16::read(source, starting_pointer + 0x0A, &Endianness::Big, "")? as usize + 0x8;
        let header_string_pool_position = binary_reader::u32::read(source, starting_pointer + 0x0C, &Endianness::Big, "")? as usize + 0x8;
        let header_data_pool_position = binary_reader::u32::read(source, starting_pointer + 0x10, &Endianness::Big, "")? as usize + 0x8;
        let header_relative_table_name_pointer = binary_reader::u32::read(source, starting_pointer + 0x14, &Endianness::Big, "")? as usize;
        let header_table_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position + header_relative_table_name_pointer as usize, "").unwrap().0;
        let header_field_count = binary_reader::u16::read(source, starting_pointer + 0x18, &Endianness::Big, "")? as usize;
        let header_row_length = binary_reader::u16::read(source, starting_pointer + 0x1A, &Endianness::Big, "")? as usize;
        let header_row_count = binary_reader::u32::read(source, starting_pointer + 0x1C, &Endianness::Big, "")? as usize;

        let pointer = starting_pointer + 0x20;

        let mut fields: Vec<CriTableField> = Vec::with_capacity(header_field_count);

        let mut field_index_pointer = 0;

        let mut row_storage_offset = 0;

        for _ in 0..header_field_count {
            let field_raw_flags = CriFieldRawFlags::from_bits(binary_reader::u8::read(source, pointer + field_index_pointer, "")?).unwrap();
            let field_flags = field_raw_flags.extract_flags();
            let field_type = field_raw_flags.extract_type();

            if !field_flags.name {
                eprintln!("Only tables with names are supported.");
                return Err(CommonBinaryError::SeeConsole());
            }

            field_index_pointer += 0x01;
            let mut field_offset = 0;

            let field_name_pointer = binary_reader::u32::read(source, pointer + field_index_pointer, &Endianness::Big, "")? as usize;
            let field_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position + field_name_pointer, "").unwrap().0;
            field_index_pointer += 0x04;

            // This value is shared between all default rows...
            let field_default_value = if field_flags.default_value {
                match field_type {
                    CriFieldType::Data => { 
                        field_index_pointer += 0x08;
                        None
                    },
                    _ => {
                        let offset = field_type.get_offset();
                        field_index_pointer += offset;
                        Some(
                            field_type.read_value(
                                source,
                                starting_pointer,
                                header_string_pool_position,
                                header_data_pool_position,
                                pointer + field_index_pointer - offset
                            )?
                        )
                    }
                }
            } else { None };

            if field_flags.row_storage {
                field_offset = row_storage_offset;
                row_storage_offset += field_type.get_offset();
            }

            let mut rows = Vec::with_capacity(header_row_count);
            for row_index in 0..header_row_count {
                let row_value = {
                    if field_flags.row_storage {
                        let value_pointer = starting_pointer + header_rows_position + (header_row_length * row_index) + field_offset;
                        field_type.read_value(source, starting_pointer, header_string_pool_position, header_data_pool_position, value_pointer)?
                    }
                    else if field_flags.default_value && field_type != CriFieldType::Data && field_type != CriFieldType::ProbablyTable {
                        field_default_value.clone().unwrap()
                    }
                    // Not even per row, and not even constant value? Then there's no storage.
                    else if !field_flags.row_storage && !field_flags.default_value {
                        field_type.clone().get_null_value()
                    } else {
                        CriFieldValue::Byte(0)
                    }
                };

                rows.push(row_value);
            }

            
            fields.push(CriTableField {
                flags: field_flags,
                ttype: field_type,
                name: field_name,
                rows,
            });
        }

        Ok(
            CriTable {
                unknown_byte: header_unknown_byte,
                encoding: CriTableHeaderEncodingType::from(header_encoding_type),
                name: header_table_name,
                fields,
            },
        )
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let mut result = Vec::new();
        
        for _ in 0..0x20 {
            result.push(0);
        }

        binary_writer::string32::write(&mut result, 0, "@UTF", "").unwrap();

        Ok(result)
    }
}
