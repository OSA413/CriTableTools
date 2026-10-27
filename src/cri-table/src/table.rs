use std::collections::HashMap;
use common_utils::get_sha256;
use itertools::Itertools;

use common_binary::{binary_reader, binary_writer, endianness::Endianness, error::CommonBinaryError};
use serde::Serialize;

use crate::{field::CriTableField, field_raw_flags::CriFieldRawFlags, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, header_encoding_type::CriTableEncodingType, table_write_helpers::{TableDataPoolBuilderEntry, TableFieldsInfoWriteResult, TableHeaderInfoWriteResult, TableHeaderWriteResult, TableRowsInfoWriteResult, TableStringPoolBuilderEntry}};

pub const UTF_SIGNATURE: [u8; 4] = *b"@UTF";

#[derive(Clone, Debug, PartialEq)]
pub struct CriTable {
    pub name: String,
    pub encoding: CriTableEncodingType,
    pub fields: Vec<CriTableField>,
    pub unknown_byte: u8,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TableJsonWrapper {
    Table(TableJson),
}


#[derive(Serialize)]
pub struct TableJson {
    pub name: String,
    pub encoding: CriTableEncodingType,
    pub fields: Vec<CriTableField>,
    pub rows: Vec<serde_json::Map<String, serde_json::Value>>,
}

impl CriTable {
    pub fn row_length(&self) -> usize {
        self.fields.iter()
            .filter(|field| field.flags.row_storage)
            .map(|field| field.ttype.get_offset())
            .sum()
    }

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
            let default_value = if field_flags.default_value {
                match field_type {
                    CriFieldType::Data => {
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
            } else {
                None
            };

            if field_flags.row_storage {
                field_offset = row_storage_offset;
                row_storage_offset += field_type.get_offset();
            }

            let mut rows = Vec::with_capacity(header_row_count);
            for row_index in 0..header_row_count {
                let row_value = {
                        match (field_flags.row_storage, field_flags.default_value) {
                        (true, _) => {
                            let value_pointer = starting_pointer + header_rows_position + (header_row_length * row_index) + field_offset;
                            field_type.read_value(source, starting_pointer, header_string_pool_position, header_data_pool_position, value_pointer)?
                        }
                        (_, true) => CriFieldValue::Default,
                        // Not even per row, and not even constant value? Then there's no storage.
                        (false, false) => CriFieldValue::Null,
                    }
                };

                rows.push(row_value);
            }

            
            fields.push(CriTableField {
                flags: field_flags,
                ttype: field_type,
                name: field_name,
                rows,
                default_value,
            });
        }

        Ok(
            CriTable {
                unknown_byte: header_unknown_byte,
                encoding: CriTableEncodingType::from(header_encoding_type),
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

        binary_writer::string::write(&mut data, 0, "@UTF", "table signature").unwrap();

        TableHeaderWriteResult {
            data,
            pointer,
            length_pointer: 0x04 - pointer,
        }
    }

    fn write_table_info(&self, pointer: usize) -> TableHeaderInfoWriteResult {
        let mut data = Vec::with_capacity(0x18);
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
            self.row_length() as u16,
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
    
    fn length_nice(value: usize) -> usize {
        value + (8 - value % 8) % 8_usize
    }

    fn write_fields_info(
        &self,
        pointer: usize,
        string_pool: &(&'static str, HashMap<String, u32>),
        data_pool: &([u8;0], HashMap<String, (u32, Vec<u8>)>),
    ) -> TableFieldsInfoWriteResult {
        let mut data: Vec<Vec<u8>> = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let mut field_data: Vec<Vec<u8>> = vec![
                vec![CriFieldRawFlags::from_flags_and_type(&field.flags, &field.ttype).bits()],
                string_pool.1.get(&field.name).unwrap().to_be_bytes().to_vec(),
            ];

            if let Some(default_value) = &field.default_value {
                field_data.push(default_value.to_bytes_for_field_info(&field.ttype, string_pool, data_pool))
            }

            data.push(field_data.concat());
        }

        TableFieldsInfoWriteResult {
            data: data.concat(),
            pointer,
        }
    }

    fn write_rows_info(
        &self,
        pointer: usize,
        string_pool: &(&'static str, HashMap<String, u32>),
        // TODO replace with the real "defaule_value" from the files
        data_pool: &([u8;0], HashMap<String, (u32, Vec<u8>)>),
    ) -> TableRowsInfoWriteResult {
        let mut data: Vec<Vec<u8>> = Vec::with_capacity(self.row_length() * self.row_count());

        for row_index in 0..self.row_count() {
            for field in &self.fields {
                if let Some(row) = &field.rows.get(row_index)
                    && field.flags.row_storage {
                    data.push(row.to_bytes_for_field_info(&field.ttype, string_pool, data_pool));
                }
            }
        }

        TableRowsInfoWriteResult {
            data: data.concat(),
            pointer,
        }
    }

    fn build_string_pool(&self) -> (&'static str, HashMap<String, u32>) {
        let default_value = "<NULL>";
        let mut result: Vec<TableStringPoolBuilderEntry> = Vec::with_capacity(
            2 + self.fields.len() * (1 + self.row_count())
        );
        
        // We will assume that the default string value is always "<NULL>", this will simplify things a bit
        result.push(TableStringPoolBuilderEntry {
            string: default_value.to_string(),
            pointer: 0,
        });

        if result.iter().find(|entry| entry.string == self.name).is_none() {
            result.push(TableStringPoolBuilderEntry {
                string: self.name.clone(),
                pointer: result.last().unwrap().pointer + result.last().unwrap().string.len() as u32 + 1,
            });
        }

        for field in &self.fields {
            if result.iter().find(|entry| entry.string == field.name).is_none() {
                result.push(TableStringPoolBuilderEntry {
                    string: field.name.clone(),
                    pointer: result.last().unwrap().pointer + result.last().unwrap().string.len() as u32 + 1,
                });
            }
        };

        for field in &self.fields {
            for row in &field.rows {
                if let CriFieldValue::String(value) = row {
                    if result.iter().find(|entry| &entry.string == value).is_none() {
                        result.push(TableStringPoolBuilderEntry {
                            string: value.clone(),
                            pointer: result.last().unwrap().pointer + result.last().unwrap().string.len() as u32 + 1,
                        });
                    }
                }
            }
        }

        (
            default_value,
            result.into_iter().map(|entry| {
                return (entry.string, entry.pointer);
            }).collect()
        )
    }

    fn build_data_pool(&self) -> ([u8; 0], HashMap<String, (u32, Vec<u8>)>) {
        let stub_default_value = [];
        let mut result: Vec<TableDataPoolBuilderEntry> = Vec::with_capacity(
            // TODO This is wrong 
            self.fields.len() * (1 + self.row_count())
        );

        for field in &self.fields {
            if let Some(default_value) = &field.default_value {
                let default_data = default_value.to_bytes_for_data(&field.ttype);
                let hash = get_sha256(&default_data);
                if result.iter().find(|entry| entry.hash == hash).is_none() {
                    // TODO: figure out how to get the "default" value for binary
                    match default_value {
                        CriFieldValue::Data(_) => {
                            result.push(TableDataPoolBuilderEntry {
                                data: default_data,
                                hash,
                                pointer: 0,
                            });
                        },
                        _ => (),
                    }
                    
                }
            }
        }

        // Just to be safe, I've never seen such value but the next code relies on the previous entry
        if let None = result.last() {
            result.push(TableDataPoolBuilderEntry {
                data: stub_default_value.to_vec(),
                pointer: 0,
                hash: get_sha256(&stub_default_value),
            });
        }

        for field in &self.fields {
            for row in &field.rows {
                if let CriFieldValue::Data(value) = row {
                    let hash = get_sha256(&value);
                    if result.iter().find(|entry| entry.hash == hash).is_none() {
                        result.push(TableDataPoolBuilderEntry {
                            data: value.clone(),
                            hash,
                            pointer: result.last().map(|last| last.data.len() + last.pointer).unwrap(),
                        });
                    }
                } else if let CriFieldValue::ProbablyTable(table_or_data) = row {
                    let data: Vec<u8> = match table_or_data {
                        TableOrData::Data(data) => data.clone(),
                        TableOrData::Table(table) => table.write().unwrap(),
                    };

                    let hash = get_sha256(&data);
                    if result.iter().find(|entry| entry.hash == hash).is_none() {
                        result.push(TableDataPoolBuilderEntry {
                            data: data,
                            hash,
                            pointer: result.last().map(|last| last.data.len() + last.pointer).unwrap(),
                        });
                    }
                }
            }
        };

        (
            stub_default_value,
            result.into_iter().map(|entry| {
                return (entry.hash, (entry.pointer as u32, entry.data));
            }).collect()
        )
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let string_pool = self.build_string_pool();
        let data_pool = self.build_data_pool();

        let signature = self.write_table_signature(0x00);
        let signature_length = signature.data.len();
        let header_info = self.write_table_info(0x08);
        let fields_info = self.write_fields_info(0, &string_pool, &data_pool);
        let rows_info = self.write_rows_info(0, &string_pool, &data_pool);
        let string_pool_max = string_pool.1.iter().max_by_key(|entry| entry.1).unwrap();
        let string_pool_size = string_pool_max.0.len() + string_pool_max.1.clone() as usize + 1;
        let mut binary_string_pool = Vec::with_capacity(string_pool_size);
        for _ in 0..string_pool_size {
            binary_string_pool.push(0);
        }
        for entry in &string_pool.1 {
            binary_writer::string::write(&mut binary_string_pool, entry.1.clone() as usize, &entry.0, "string pool entry").unwrap();
        }

        let binary_data_pool = data_pool.1.iter().sorted_by_key(|entry| entry.1.0).map(|entry| &entry.1.1).map(|x| x.clone()).concat().iter().map(|x| x.clone()).collect::<Vec<u8>>();
        
        let row_position = signature.data.len()
            + header_info.data.len()
            + fields_info.data.len()
            - signature.data.len();

        let string_pool_position = row_position + rows_info.data.len();
        let data_pool_position = CriTable::length_nice(string_pool_position + string_pool_size) as u32;

        let mut before_data = [
            signature.data,
            header_info.data,
            fields_info.data,
            rows_info.data,
            binary_string_pool,
        ].concat();

        for _ in 0..CriTable::length_nice(before_data.len()) - before_data.len() {
            before_data.push(0);
        }

        let mut result = [
            before_data,
            binary_data_pool,
        ].concat();

        let result_length = result.len();
        
        binary_writer::string::write(&mut result, 0, "@UTF", "").unwrap();
        binary_writer::u32::write(
            &mut result,
            0x04,
            (result_length - signature_length) as u32,
            &Endianness::Big,
            ""
        ).unwrap();
        binary_writer::u16::write(
            &mut result,
            0x0A,
            row_position as u16,
            &Endianness::Big,
            ""
        ).unwrap();
        binary_writer::u32::write(
            &mut result,
            0x0C,
            string_pool_position as u32,
            &Endianness::Big,
            ""
        ).unwrap();
        binary_writer::u32::write(
            &mut result,
            0x10,
            data_pool_position as u32,
            &Endianness::Big,
            ""
        ).unwrap();
        binary_writer::u32::write(
            &mut result,
            0x14,
            string_pool.1.get(&self.name).unwrap().clone(),
            &Endianness::Big,
            ""
        ).unwrap();

        Ok(result)
    }

    pub fn to_json(&self) -> TableJson {
        TableJson {
            name: self.name.clone(),
            encoding: self.encoding,
            fields: self.fields.clone(),
            rows: self.dump_rows_to_json(),
        }
    }

    pub fn dump_rows_to_json(&self) -> Vec<serde_json::Map<String, serde_json::Value>> {
        let mut rows = Vec::with_capacity(self.row_count());
        for row_index in 0..self.row_count() {
            let mut values = serde_json::Map::new();
            for field in &self.fields {
                let json_value = match &field.rows[row_index] {
                    CriFieldValue::ProbablyTable(table_or_data) => match table_or_data {
                        TableOrData::Table(cri_table) => serde_json::to_value(TableJsonWrapper::Table(cri_table.to_json())),
                        value => serde_json::to_value(value),
                    },
                    value => serde_json::to_value(value),
                }.unwrap();
                values.insert(field.name.clone(), json_value);
            }
            rows.push(values);
        }
        rows
    }
}
