use common_binary::error::CommonBinaryError;
use cri_table::table::CriTable;

use crate::{aax::archive::CriCsbAaxArchive, adpcm_wii::{archive::CriCsbAdpcmWiiArchive, entry::CriAdpcmWiiEntry}, traits::{FromCriTable, ToCriTable}};

#[derive(Debug)]
pub enum CriCsbSoundArchiveEnum {
    Aax(CriCsbAaxArchive),
    AdpcmWii(CriAdpcmWiiEntry),
    Stream,
}

impl FromCriTable for CriCsbSoundArchiveEnum {
    fn from_table(root_table: &CriTable) -> Result<CriCsbSoundArchiveEnum, CommonBinaryError> {
        let data = match root_table.name.as_str() {
            "AAX" => CriCsbSoundArchiveEnum::Aax(CriCsbAaxArchive::from_table(root_table, &root_table.name.to_string()).unwrap()),
            // "SWLPCM" => CriSoundArchiveEnum::Wav,
            "ADPCM_WII" => CriCsbSoundArchiveEnum::AdpcmWii(
                CriCsbAdpcmWiiArchive::read(root_table).unwrap(),
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
        let result = match self {
            CriCsbSoundArchiveEnum::Aax(_cri_csb_aax_archive) => todo!(),
            CriCsbSoundArchiveEnum::AdpcmWii(entry) => {
                entry.to_table().unwrap()
            },
            CriCsbSoundArchiveEnum::Stream => todo!(),
        };

        Ok(result)
    }
}