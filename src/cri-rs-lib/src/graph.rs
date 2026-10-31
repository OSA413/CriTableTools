use common_binary::error::CommonBinaryError;
use cri_table::{field_dto::CriFieldDto, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, table::CriTable};
use serde::{Deserialize, Serialize};

use crate::traits::{FromCriTable, ToCriTable};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGraphWrapper {
    pub original_string_pool: Vec<String>,
    pub entries: Vec<CriGraph>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGraph {
    pub ttype: u8,
    pub imax: f32,
    pub imin: f32,
    pub omax: f32,
    pub omin: f32,
    pub points: CriGraphPoints,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGraphPointsEntry {
    pub in_: u16,
    pub out: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGraphPoints {
    pub original_string_pool: Vec<String>,
    pub entries: Vec<CriGraphPointsEntry>,
}

impl FromCriTable for CriGraphPoints {
    fn from_table(table: &CriTable) -> Result<CriGraphPoints, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());

        for row_index in 0..table.row_count() {
            entries.push(CriGraphPointsEntry {
                in_: table.get_field("in").unwrap().to_u16(row_index).unwrap(),
                out: table.get_field("out").unwrap().to_u16(row_index).unwrap(),
            });
        }

        Ok(CriGraphPoints {
            original_string_pool: table.original_string_pool.clone(),
            entries
        })
    }
}

impl ToCriTable for CriGraphPoints {
    fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut table = CriTable::new(
            "TBLIPT",
            &self.original_string_pool,
            &[
                CriFieldDto::new("in", CriFieldType::UInt16, None),
                CriFieldDto::new("out", CriFieldType::UInt16, None),
            ],
            self.entries.len(),
        );
        
        for entry in &self.entries {
            table.fields[0].rows.push(CriFieldValue::UInt16(entry.in_));
            table.fields[1].rows.push(CriFieldValue::UInt16(entry.out));
        }

        Ok(table)
    }
}

impl ToCriTable for CriGraphWrapper {
    fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut table = CriTable::new(
            "TBLIGR",
            &self.original_string_pool,
            &[
                CriFieldDto::new("type", CriFieldType::Byte, None),
                CriFieldDto::new("imax", CriFieldType::Single, None),
                CriFieldDto::new("imin", CriFieldType::Single, None),
                CriFieldDto::new("omax", CriFieldType::Single, None),
                CriFieldDto::new("omin", CriFieldType::Single, None),
                CriFieldDto::new("points", CriFieldType::ProbablyTable, None),
            ],
            self.entries.len(),
        );

        for entry in &self.entries {
            table.fields[0].rows.push(CriFieldValue::Byte(entry.ttype));
            table.fields[1].rows.push(CriFieldValue::Single(entry.imax));
            table.fields[2].rows.push(CriFieldValue::Single(entry.imin));
            table.fields[3].rows.push(CriFieldValue::Single(entry.omax));
            table.fields[4].rows.push(CriFieldValue::Single(entry.omin));
            table.fields[5].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(entry.points.to_table().unwrap().clone())));
        }

        Ok(table)
    }
}

impl FromCriTable for CriGraphWrapper {
    fn from_table(table: &CriTable) -> Result<CriGraphWrapper, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());

        for row_index in 0..table.row_count() {
            entries.push(CriGraph {
                ttype: table.get_field("type").unwrap().to_u8(row_index).unwrap(),
                imax: table.get_field("imax").unwrap().to_f32(row_index).unwrap(),
                imin: table.get_field("imin").unwrap().to_f32(row_index).unwrap(),
                omax: table.get_field("omax").unwrap().to_f32(row_index).unwrap(),
                omin: table.get_field("omin").unwrap().to_f32(row_index).unwrap(),
                points: CriGraphPoints::from_table(table.get_field("points").unwrap().to_table(row_index).unwrap()).unwrap(),
            });
        }

        Ok(CriGraphWrapper {
            original_string_pool: table.original_string_pool.clone(),
            entries,
        })
    }
}
