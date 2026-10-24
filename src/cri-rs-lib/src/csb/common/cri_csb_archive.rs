use std::{fs, path::Path};

use common_binary::error::CommonBinaryError;
use cri_table::{field::CriTableField, field_flags::CriFieldFlags, field_type::CriFieldType, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType, table::{CriTable, UTF_SIGNATURE}};

use crate::csb::{common::cri_csb_adpcm_wii_sound_element::CriCsbAdpcmWiiSoundElementData, games::black_knight::archive::{CriCsbAdpcmWiiCue, CriCsbAdpcmWiiInfo, CriCsbAdpcmWiiIsaac, CriCsbAdpcmWiiSynth, CriCsbAdpcmWiiVoiceLimitGroup, CriGraph}};

pub struct CriCsbTablesBuilder {
    pub info: Option<CriTable>,
    pub cue: Option<CriTable>,
    pub synth: Option<CriTable>,
    pub sound_element: Option<CriTable>,
    pub isaac: Option<CriTable>,
    pub voice_limit_group: Option<CriTable>,
}

pub struct CriCsbAdpcmWiiSoundElement {
    pub name: String,
    pub data: CriCsbAdpcmWiiSoundElementData,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}

pub fn get_csb_fields(csb_table: &CriTable) -> CriCsbTablesBuilder {
    let mut info = None;
    let mut cue = None;
    let mut synth = None;
    let mut sound_element = None;
    let mut isaac = None;
    let mut voice_limit_group = None;

    for row_index in 0..csb_table.row_count() {
        match csb_table.get_field("name").unwrap().to_string(row_index).unwrap().as_str() {
            "INFO" => info = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            "CUE" => cue = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            "SYNTH" => synth = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            "SOUND_ELEMENT" => sound_element = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            "ISAAC" => isaac = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            "VOICE_LIMIT_GROUP" => voice_limit_group = Some(csb_table.get_field("utf").unwrap().to_table(row_index).unwrap().to_owned()),
            table_name => println!("Detacted unknown table with name [{}]", table_name),
        }
    }

    CriCsbTablesBuilder {
        info,
        cue,
        synth,
        sound_element,
        isaac,
        voice_limit_group,
    }
}
