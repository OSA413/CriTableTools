use common_binary::error::CommonBinaryError;
use cri_table::{encoding_type::CriTableEncodingType, table::CriTable};

use crate::{aax::archive::CriAaxArchive, adpcm_wii::archive::{CriAdpcmWiiArchive, CriAdpcmWiiArchiveWrapper}};

#[derive(Debug)]
pub enum CriSoundArchiveEnum {
    Aax(CriAaxArchive),
    AdpcmWii(CriAdpcmWiiArchiveWrapper),
}

#[derive(Debug)]
pub struct CriSoundArchive {
    pub name: String,
    pub data: CriSoundArchiveEnum,
}

impl CriSoundArchive {
    pub fn read(root_table: &CriTable, name: &str) -> Result<CriSoundArchive, CommonBinaryError> {
        let data = match root_table.name.as_str() {
            "AAX" => CriSoundArchiveEnum::Aax(
                match CriAaxArchive::read(root_table, &root_table.name.to_string()).unwrap().data {
                    CriSoundArchiveEnum::Aax(cri_aax_archive) => cri_aax_archive,
                    CriSoundArchiveEnum::AdpcmWii(_) => panic!("Not a CriAaxArchive"),
                }
            ),
            // "SWLPCM" => CriSoundArchiveEnum::Wav,
            "ADPCM_WII" => CriSoundArchiveEnum::AdpcmWii(
                CriAdpcmWiiArchive::read(root_table).unwrap(),
            ),
            value => {
                eprintln!("Unknown Sound type [{value}]");
                return Err(CommonBinaryError::SeeConsole());
            },
        };

        Ok(CriSoundArchive {
            name: name.to_string(),
            data,
        })
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let result = CriTable {
            name: "AAX".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            fields: vec![],
            unknown_byte: 0,
        };

        result.write(&Vec::<String>::new())
    }
}
