use std::fmt::{self, Formatter};

use common_binary::error::CommonBinaryError;
use cri_table::{field_dto::CriFieldDto, field_type::CriFieldType, field_value::CriFieldValue, table::CriTable};

use crate::loop_flag::CriLoopFlag;

#[derive(Clone)]
pub struct CriAdpcmWiiEntry {
    pub sfreq: u32,
    pub nsmpl: u32,
    pub nch: u8,
    pub loop_flag: CriLoopFlag,
    pub data: Vec<u8>,
    pub header: Vec<u8>,
}

impl CriAdpcmWiiEntry {
    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut inner_table = CriTable::new(
            "ADPCM_WII",
            &vec![],
            &[
                CriFieldDto::new("sfreq", CriFieldType::UInt32, None),
                CriFieldDto::new("nsmpl", CriFieldType::UInt32, None),
                CriFieldDto::new("nch", CriFieldType::Byte, None),
                CriFieldDto::new("lpflg", CriFieldType::Byte, None),
                CriFieldDto::new("data", CriFieldType::Data, None),
                CriFieldDto::new("header", CriFieldType::Data, None),
            ],
            1,
        );

        inner_table.fields[0].rows.push(CriFieldValue::UInt32(self.sfreq));
        inner_table.fields[1].rows.push(CriFieldValue::UInt32(self.nsmpl));
        inner_table.fields[2].rows.push(CriFieldValue::Byte(self.nch));
        inner_table.fields[3].rows.push(CriFieldValue::Byte(self.loop_flag.clone() as u8));
        inner_table.fields[4].rows.push(CriFieldValue::Data(self.data.clone()));
        inner_table.fields[5].rows.push(CriFieldValue::Data(self.header.clone()));

        Ok(inner_table)
    }
}

impl fmt::Debug for CriAdpcmWiiEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAdpcmWiiEntry {{ flag: {:?}, data_length: {} }}", self.loop_flag, self.data.len())
    }
}