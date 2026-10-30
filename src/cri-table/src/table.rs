use std::collections::{HashMap, HashSet};
use common_utils::get_sha256;
use itertools::{Itertools};

use common_binary::{binary_reader, binary_writer, endianness::Endianness, error::CommonBinaryError};
use serde::Serialize;

use crate::{encoding_type::CriTableEncodingType, field::CriTableField, field_flags_simple::CriFieldFlagsSimple::{self}, field_raw_flags::CriFieldRawFlags, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, table_write_helpers::{TableDataPoolBuilderEntry, TableStringPoolBuilderEntry}};

pub const UTF_SIGNATURE: [u8; 4] = *b"@UTF";

#[derive(Clone, Debug, PartialEq)]
pub struct CriTable {
    pub original_string_pool: Vec<String>,
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
        if self.row_count() == 0 {
            return 0;
        }
        let mut result = 0;
        for field in &self.fields {
            if field.default_value.is_none() {
                result += field.ttype.get_offset()
            }
        }

        result
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

    pub fn read(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4])
        -> Result<CriTable, CommonBinaryError> {
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
                    if field_flags.row_storage {
                        let value_pointer = starting_pointer + header_rows_position + (header_row_length * row_index) + field_offset;
                        field_type.read_value(source, starting_pointer, header_string_pool_position, header_data_pool_position, value_pointer)?
                    } else {
                        CriFieldValue::Default
                    }
                };

                rows.push(row_value);
            }

            fields.push(CriTableField {
                ttype: field_type,
                name: field_name,
                rows,
                default_value,
            });
        }

        let mut original_string_pool_set = HashSet::new();

        original_string_pool_set.insert("<NULL>");
        original_string_pool_set.insert(&header_table_name);

        for field in &fields {
            if let Some(default_value) = &field.default_value
                && let CriFieldValue::String(default_string) = default_value {
                original_string_pool_set.insert(default_string);
            }
            original_string_pool_set.insert(&field.name);
            for row in &field.rows {
                if let CriFieldValue::String(string) = row {
                    original_string_pool_set.insert(string);
                }
            }
        }

        let mut original_string_pool = Vec::new();
        let mut pointer = starting_pointer + header_string_pool_position;
        for _ in 0..original_string_pool_set.len() {
            let string_read = binary_reader::string::read(source, pointer, "").unwrap();
            pointer = string_read.1;
            original_string_pool.push(string_read.0);
        }

        Ok(
            CriTable {
                original_string_pool,
                unknown_byte: header_unknown_byte,
                encoding: CriTableEncodingType::from(header_encoding_type),
                name: header_table_name,
                fields,
            },
        )
    }

    fn table_signature_to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0; 0x8];
        binary_writer::string::write(&mut data, 0, "@UTF", "table signature").unwrap();
        data
    }

    fn table_info_to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0; 0x18];
        binary_writer::u16::write(
            &mut data,
            0x10,
            self.fields.len() as u16,
            &Endianness::Big, ""
        ).unwrap();

        binary_writer::u16::write(
            &mut data,
            0x12,
            self.row_length() as u16,
            &Endianness::Big,
            "",
        ).unwrap();

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

        data
    }
    
    // TODO move to the common_binary
    pub fn length_nice(value: usize) -> usize {
        value + (8 - value % 8) % 8_usize
    }

    fn fields_info_to_bytes(
        &self,
        string_pool: &HashMap<String, u32>,
        data_pool: &HashMap<String, (u32, Vec<u8>)>,
    ) -> Vec<u8> {
        let mut data: Vec<Vec<u8>> = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let simple_flag = field.actual_flags();

            let mut field_data: Vec<Vec<u8>> = vec![
                vec![CriFieldRawFlags::from_flags_and_type(&simple_flag, &field.ttype).bits()],
                string_pool.get(&field.name).unwrap().to_be_bytes().to_vec(),
            ];

            if let CriFieldFlagsSimple::Default = &simple_flag
                && let Some(default_value) = &field.default_value {
                field_data.push(default_value.to_bytes_for_field_info(&field.ttype, string_pool, data_pool))
            }

            data.push(field_data.concat());
        }

        data.concat()
    }

    fn rows_to_bytes(
        &self,
        string_pool: &HashMap<String, u32>,
        data_pool: &HashMap<String, (u32, Vec<u8>)>,
    ) -> Vec<u8> {
        let mut data: Vec<Vec<u8>> = Vec::with_capacity(self.row_length() * self.row_count());

        for row_index in 0..self.row_count() {
            for field in &self.fields {
                let actual_flag = field.actual_flags();
                if actual_flag == CriFieldFlagsSimple::RowStorage {
                    data.push(field.rows[row_index].to_bytes_for_field_info(&field.ttype, string_pool, data_pool));
                }
            }
        }

        data.concat()
    }

    fn build_string_pool(&self, original_string_pool: &Vec<String>) -> HashMap<String, u32> {
        let mut result: Vec<TableStringPoolBuilderEntry> = CriTable::build_string_pool_from_original(original_string_pool);
        
        // We will assume that the default string value is always "<NULL>", this will simplify things a bit
        if result.iter().find(|x| x.string == "<NULL>").is_none() {
            result.insert(0, TableStringPoolBuilderEntry {
                string: "<NULL>".to_string(),
                pointer: 0,
            });
        }

        if result.iter().find(|entry| entry.string == self.name).is_none() {
            result.push(TableStringPoolBuilderEntry {
                string: self.name.clone(),
                pointer: result.last().map(|last | last.pointer + last.string.len() as u32).unwrap_or(0) + 1,
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
                if let CriFieldValue::String(value) = row
                    && result.iter().find(|entry| &entry.string == value).is_none() {
                        result.push(TableStringPoolBuilderEntry {
                            string: value.clone(),
                            pointer: result.last().map(|last | last.pointer + last.string.len() as u32).unwrap_or(0) + 1,
                        });
                    }
            }
        }

        result.into_iter().map(|entry| {
            (entry.string, entry.pointer)
        }).collect()
    }

    fn build_data_pool(&self) -> HashMap<String, (u32, Vec<u8>)> {
        let mut result: Vec<TableDataPoolBuilderEntry> = Vec::new();

        for field in &self.fields {
            for row in &field.rows {
                if let CriFieldValue::Data(value) = row {
                    let mut data = value.clone();
                    data.extend(std::iter::repeat_n(0, CriTable::length_nice(data.len()) - data.len()));

                    let hash = get_sha256(value);
                    if result.iter().find(|entry| entry.hash == hash).is_none() {
                        result.push(TableDataPoolBuilderEntry {
                            data: value.clone(),
                            hash,
                            pointer: result.last().map(|last| CriTable::length_nice(last.data.len()) + last.pointer).unwrap_or(0),
                        });
                    }
                } else if let CriFieldValue::ProbablyTable(table_or_data) = row {
                    let data = match table_or_data {
                        TableOrData::Data(data) => data.clone(),
                        TableOrData::Table(table) => table.write().unwrap(),
                    };

                    let hash = get_sha256(&data);
                    if result.iter().find(|entry| entry.hash == hash).is_none() {
                        result.push(TableDataPoolBuilderEntry {
                            data,
                            hash,
                            pointer: result.last().map(|last| CriTable::length_nice(last.data.len()) + last.pointer).unwrap_or(0),
                        });
                    }
                }
            }
        };

        result.into_iter().map(|entry| {
            (entry.hash, (entry.pointer as u32, entry.data))
        }).collect()
    }

    pub fn build_string_pool_from_original(original_string_pool: &Vec<String>) -> Vec<TableStringPoolBuilderEntry> {
        let mut result = Vec::with_capacity(original_string_pool.len());
        let mut current_pointer = 0;
        for entry in original_string_pool {
            result.push(TableStringPoolBuilderEntry {
                pointer: current_pointer as u32,
                string: entry.clone()
            });
            current_pointer += entry.len() + 1;
        }
        result
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let string_pool = self.build_string_pool(&self.original_string_pool);
        let data_pool = self.build_data_pool();

        let signature = self.table_signature_to_bytes();
        let signature_length = signature.len();
        let header = self.table_info_to_bytes();
        let fields = self.fields_info_to_bytes(&string_pool, &data_pool);
        let rows_info = self.rows_to_bytes(&string_pool, &data_pool);
        let binary_string_pool = string_pool.iter().sorted_by_key(|entry| entry.1).map(|entry| [entry.0.clone().into_bytes(), vec![0]].concat()).concat().to_vec();
        let binary_data_iter = data_pool.iter().sorted_by_key(|entry| entry.1.0).map(|entry| &entry.1.1).cloned();

        let mut binary_data_pool = Vec::new();
        for entry in binary_data_iter {
            binary_data_pool.extend_from_slice(&entry);
            binary_data_pool.extend(std::iter::repeat_n(0, CriTable::length_nice(binary_data_pool.len()) - binary_data_pool.len()))
        }
        
        let row_position = signature.len()
            + header.len()
            + fields.len()
            - signature.len();

        let string_pool_position = row_position + rows_info.len();

        let mut before_data = [
            signature,
            header,
            fields,
            rows_info,
            binary_string_pool,
        ].concat();

        before_data.extend(std::iter::repeat_n(0, CriTable::length_nice(before_data.len()) - before_data.len()));

        let data_pool_position = before_data.len() - signature_length;

        let mut result = [
            before_data,
            binary_data_pool,
        ].concat();

        let result_length = result.len();

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
            *string_pool.get(&self.name).unwrap(),
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
