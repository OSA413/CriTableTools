use std::collections::HashMap;

use common_binary::{binary_reader, binary_writer, endianness::Endianness, error::CommonBinaryError};

use crate::{field::CriTableField, field_raw_flags::CriFieldRawFlags, field_type::CriFieldType, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType, table_write_helpers::{TableFieldsInfoWriteResult, TableHeaderInfoWriteResult, TableHeaderWriteResult, TableStringPoolBUilderEntry}};

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
                        // TODO: check
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
                default_value: field_default_value,
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

    fn write_table_signature(&self, pointer: usize) -> TableHeaderWriteResult {
        let mut data = Vec::with_capacity(0x8);
        for _ in 0..0x08 {
            data.push(0 as u8);
        }

        binary_writer::string32::write(&mut data, 0, "@UTF", "table signature").unwrap();

        // Length, excluding the table header length (8)
        // So, this is probably the length of the inner table

        TableHeaderWriteResult {
            data,
            pointer,
            length_pointer: 0x04 - pointer,
        }
    }

    fn write_table_info(&self, pointer: usize) -> TableHeaderInfoWriteResult {
        let mut data = Vec::new();
        for _ in 0..0x18 {
            data.push(0 as u8);
        }

        // u8 unknown_byte, always 0
        // u8 encoding_type, only 0 (shift-jis) for now
        // u16 row_position
        let row_position_pointer = 0x02;
        // u32 string_pool_position
        let string_pool_position_pointer = 0x04;
        // u32 data_pool_position
        let data_pool_position_pointer = 0x08;
        // u32 table_name_position
        let table_name_position_pointer = 0x0C;
        // field_count
        binary_writer::u16::write(&mut data, 0x10, self.fields.len() as u16, &Endianness::Big, "").unwrap();
        // row_length
        binary_writer::u16::write(
            &mut data,
            0x12,
            self.fields.iter().map(|field| field.ttype.get_offset() as u16).sum(),
            &Endianness::Big,
            "",
        ).unwrap();
        // row_count
        binary_writer::u32::write(
            &mut data,
            0x14,
            match self.fields.first() {
                Some(field) => field.rows.len() as u32,
                None => 0
            },
            &Endianness::Big,
            "",
        ).unwrap();

        TableHeaderInfoWriteResult {
            data,
            pointer,
            row_position_pointer,
            string_pool_position_pointer,
            data_pool_position_pointer,
            table_name_position_pointer,
        }
    }

    fn write_fields_info(&self, pointer: usize, string_pool: HashMap<String, u32>) -> TableFieldsInfoWriteResult {
        let mut data: Vec<Vec<u8>> = Vec::new();

        for field in &self.fields {
            let field_data: Vec<Vec<u8>> = vec![
                CriFieldRawFlags::from_flags_and_type(&field.flags, &field.ttype).bits().to_be_bytes().to_vec(),
                string_pool.get(&field.name).unwrap().to_be_bytes().to_vec(),
            ];
            data.push(field_data.concat());
        }

        TableFieldsInfoWriteResult {
            data: data.concat(),
            length: 0,
            pointer,
        }
    }

    fn build_string_pool(&self) -> HashMap<String, u32> {
        let mut result: Vec<TableStringPoolBUilderEntry> = Vec::new();
        for field in &self.fields {
            if result.iter().find(|entry| entry.string == field.name).is_none() {
                if let Some(default_value) = &field.default_value {
                    match default_value {
                        CriFieldValue::String(value) => {
                            result.push(TableStringPoolBUilderEntry {
                                string: value.clone(),
                                pointer: 0,
                            });
                        },
                        _ => (),
                    }
                    
                }
            }
        }
        for field in &self.fields {
            if result.iter().find(|entry| entry.string == field.name).is_none() {
                result.push(TableStringPoolBUilderEntry {
                    string: field.name.clone(),
                    pointer: result.last().unwrap().pointer + result.last().unwrap().string.len() as u32 + 1,
                });
            }
        };

        result.into_iter().map(|entry| {
            return (entry.string, entry.pointer);
        }).collect()
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let string_pool = self.build_string_pool();
        // data_pool

        let signature = self.write_table_signature(0x00);
        let header_info = self.write_table_info(0x08);
        let fields_info = self.write_fields_info(0, string_pool);
        // let rows_info = self.write_rows_info();

        let mut result = [
            signature.data,
            header_info.data,
            fields_info.data,
        ].concat();

        binary_writer::string32::write(&mut result, 0, "@UTF", "").unwrap();

        Ok(result)
    }
}
