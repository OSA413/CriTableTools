use common_binary::error::CommonBinaryError;
use cri_table::{encoding_type::CriTableEncodingType, table::CriTable};

use crate::{aax::archive::CriAaxArchive, adpcm_wii::archive::{CriArchive}, traits::{FromCriTable, ToCriTable}};

#[derive(Debug)]
pub enum CriCsbSoundArchiveEnum {
    Aax(CriAaxArchive),
    AdpcmWii(CriArchiveWrapper),
}

impl FromCriTable for CriCsbSoundArchiveEnum {
    fn from_table(root_table: &CriTable) -> Result<CriCsbSoundArchiveEnum, CommonBinaryError> {
        let data = match root_table.name.as_str() {
            "AAX" => CriCsbSoundArchiveEnum::Aax(CriAaxArchive::from_table(root_table, &root_table.name.to_string()).unwrap()),
            // "SWLPCM" => CriSoundArchiveEnum::Wav,
            "ADPCM_WII" => CriCsbSoundArchiveEnum::AdpcmWii(
                CriArchive::read(root_table).unwrap(),
            ),
            value => {
                eprintln!("Unknown Sound type [{value}]");
                return Err(CommonBinaryError::SeeConsole());
            },
        };

        Ok(data)
    }
}

impl ToCriTable for CriCsbSoundArchiveEnum {
    fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let result = CriTable {
            original_string_pool: vec![], //TODO
            name: "AAX".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            fields: vec![],
            unknown_byte: 0,
        };

        Ok(result)
    }
}