use std::{fs, path::Path};

use common_binary::error::CommonBinaryError;
use cri_table::{field::CriTableField, field_flags::CriFieldFlags, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, encoding_type::CriTableEncodingType, table::{CriTable, UTF_SIGNATURE}};

use crate::{aax::archive::CriAaxArchive, cpk::sound::archive::CriCpkArchive, csb::{common::{cri_csb_aax_sound_element::{CriCsbAaxSoundElement, CriCsbAaxSoundElementData}, cri_csb_archive::get_csb_fields}, games::{black_knight::archive::CriGraph, episodes::archive::{CriCsbAaxCue, CriCsbAaxInfo, CriCsbAaxIsaac, CriCsbAaxSynth, CriCsbAaxVoiceLimitGroup}}}, find_related_cpk, sound_archive::CriSoundArchiveEnum};

pub struct CriCsbAaxArchive {
    pub info: Vec<CriCsbAaxInfo>,
    pub cue: Vec<CriCsbAaxCue>,
    pub synth: Vec<CriCsbAaxSynth>,
    pub sound_element: Vec<CriCsbAaxSoundElement>,
    pub isaac: Vec<CriCsbAaxIsaac>,
    pub voice_limit_group: Vec<CriCsbAaxVoiceLimitGroup>,
}

pub fn get_aax_sound_data(sdl_table: &CriTable, cpk: Option<CriCpkArchive>) -> Result<Vec<CriCsbAaxSoundElement>, CommonBinaryError> {
    let is_streaming = sdl_table.get_field("stmflg").unwrap().to_bool(0).unwrap();
    // if sdl_table.get_field("fmt").unwrap().to_bool(0).unwrap() {
    //     eprintln!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
    //     return Err(CommonBinaryError::SeeConsole());
    // }

    match (is_streaming, cpk) {
        (true, None) => {
            eprintln!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
            return Err(CommonBinaryError::SeeConsole());
        },
        (true, Some(_cpk)) => {
            panic!("CPK not yet implemented"); //return Ok(vec![CriCsbSoundElement::Stream(cpk)]);
        }
        (false, _) => {
            let mut sound_entries = Vec::with_capacity(sdl_table.row_count());
            for sdl_row_index in 0..sdl_table.row_count() {
                let sdl_name = sdl_table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();                
                let aax_table = sdl_table.get_field("data").unwrap().to_table(sdl_row_index).unwrap();
                sound_entries.push(CriCsbAaxSoundElement {
                    name: sdl_name.clone(),
                    data: CriCsbAaxSoundElementData::Aax(
                        match CriAaxArchive::read(aax_table, &sdl_name).unwrap().data {
                            CriSoundArchiveEnum::Aax(cri_aax_archive) => cri_aax_archive,
                            _ => panic!("cpk data crash"),
                        }
                    ),
                    fmt: sdl_table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap(),
                    nch: sdl_table.get_field("nch").unwrap().to_u8(sdl_row_index).unwrap(),
                    stmflg: sdl_table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap(),
                    sfreq: sdl_table.get_field("sfreq").unwrap().to_u32(sdl_row_index).unwrap(),
                    nsmpl: sdl_table.get_field("nsmpl").unwrap().to_u32(sdl_row_index).unwrap(),
                });
            }
            return Ok(sound_entries);
        },
    };
}

impl CriCsbAaxArchive {
    pub fn read(path: &Path) -> Result<CriCsbAaxArchive, CommonBinaryError> {
        let file_content = fs::read(path).unwrap();
        let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();

        let cpk_path = find_related_cpk(&path);
        let cpk_data = cpk_path.map(|path| fs::read(path).unwrap());
        let cpk = cpk_data.map(|data| CriCpkArchive::read(&data, 0).unwrap());

        let parent_fields = get_csb_fields(&csb_table);

        return Ok(
            CriCsbAaxArchive {
                info: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.info {
                        for row_index in 0..table.row_count() {
                            result.push(CriCsbAaxInfo {
                                data_fmt_version: table.get_field("DataFmtVer").unwrap().to_u32(row_index).unwrap(),
                                ext_size: table.get_field("ExtSize").unwrap().to_u32(row_index).unwrap(),
                            });
                        }
                    }
                    result
                },
                cue: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.cue {
                         for row_index in 0..table.row_count() {
                            result.push(CriCsbAaxCue {
                                name: table.get_field("name").unwrap().to_string(row_index).unwrap(),
                                id: table.get_field("id").unwrap().to_u32(row_index).unwrap(),
                                synth: table.get_field("synth").unwrap().to_string(row_index).unwrap(),
                                udata: table.get_field("udata").unwrap().to_string(row_index).unwrap(),
                                flags: table.get_field("flags").unwrap().to_u8(row_index).unwrap(),
                            });
                        }
                    }
                    result
                },
                synth: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.synth {
                        for row_index in 0..table.row_count() {
                            result.push(CriCsbAaxSynth {
                                // Special thanks to VSCode's multi-cursor feature
                                synname: table.get_field("synname").unwrap().to_string(row_index).unwrap(),
                                syntype: table.get_field("syntype").unwrap().to_u8(row_index).unwrap(), 
                                cmplxtype: table.get_field("cmplxtype").unwrap().to_u8(row_index).unwrap(), 
                                lnkname: table.get_field("lnkname").unwrap().to_string(row_index).unwrap(), 
                                issetname: table.get_field("issetname").unwrap().to_string(row_index).unwrap(), 
                                volume: table.get_field("volume").unwrap().to_i16(row_index).unwrap(), 
                                pitch: table.get_field("pitch").unwrap().to_i16(row_index).unwrap(), 
                                dlytim: table.get_field("dlytim").unwrap().to_u32(row_index).unwrap(), 
                                s_cntrl: table.get_field("s_cntrl").unwrap().to_u8(row_index).unwrap(), 
                                eg_dly: table.get_field("eg_dly").unwrap().to_u16(row_index).unwrap(), 
                                eg_atk: table.get_field("eg_atk").unwrap().to_u16(row_index).unwrap(), 
                                eg_hld: table.get_field("eg_hld").unwrap().to_u16(row_index).unwrap(), 
                                eg_dcy: table.get_field("eg_dcy").unwrap().to_u16(row_index).unwrap(), 
                                eg_rel: table.get_field("eg_rel").unwrap().to_u16(row_index).unwrap(), 
                                eg_sus: table.get_field("eg_sus").unwrap().to_u16(row_index).unwrap(), 
                                f_type: table.get_field("f_type").unwrap().to_u8(row_index).unwrap(), 
                                f_cof1: table.get_field("f_cof1").unwrap().to_u16(row_index).unwrap(), 
                                f_cof2: table.get_field("f_cof2").unwrap().to_u16(row_index).unwrap(), 
                                f_reso: table.get_field("f_reso").unwrap().to_u16(row_index).unwrap(), 
                                f_roff: table.get_field("f_roff").unwrap().to_u8(row_index).unwrap(), 
                                dryoname: table.get_field("dryoname").unwrap().to_string(row_index).unwrap(), 
                                mtxrtr: table.get_field("mtxrtr").unwrap().to_string(row_index).unwrap(), 
                                dry0: table.get_field("dry0").unwrap().to_u16(row_index).unwrap(), 
                                dry1: table.get_field("dry1").unwrap().to_u16(row_index).unwrap(), 
                                dry2: table.get_field("dry2").unwrap().to_u16(row_index).unwrap(), 
                                dry3: table.get_field("dry3").unwrap().to_u16(row_index).unwrap(), 
                                dry4: table.get_field("dry4").unwrap().to_u16(row_index).unwrap(), 
                                dry5: table.get_field("dry5").unwrap().to_u16(row_index).unwrap(), 
                                dry6: table.get_field("dry6").unwrap().to_u16(row_index).unwrap(), 
                                dry7: table.get_field("dry7").unwrap().to_u16(row_index).unwrap(), 
                                wetoname: table.get_field("wetoname").unwrap().to_string(row_index).unwrap(), 
                                wet0: table.get_field("wet0").unwrap().to_u16(row_index).unwrap(), 
                                wet1: table.get_field("wet1").unwrap().to_u16(row_index).unwrap(), 
                                wet2: table.get_field("wet2").unwrap().to_u16(row_index).unwrap(), 
                                wet3: table.get_field("wet3").unwrap().to_u16(row_index).unwrap(), 
                                wet4: table.get_field("wet4").unwrap().to_u16(row_index).unwrap(), 
                                wet5: table.get_field("wet5").unwrap().to_u16(row_index).unwrap(), 
                                wet6: table.get_field("wet6").unwrap().to_u16(row_index).unwrap(), 
                                wet7: table.get_field("wet7").unwrap().to_u16(row_index).unwrap(), 
                                wcnct0: table.get_field("wcnct0").unwrap().to_string(row_index).unwrap(), 
                                wcnct1: table.get_field("wcnct1").unwrap().to_string(row_index).unwrap(), 
                                wcnct2: table.get_field("wcnct2").unwrap().to_string(row_index).unwrap(), 
                                wcnct3: table.get_field("wcnct3").unwrap().to_string(row_index).unwrap(), 
                                wcnct4: table.get_field("wcnct4").unwrap().to_string(row_index).unwrap(), 
                                wcnct5: table.get_field("wcnct5").unwrap().to_string(row_index).unwrap(), 
                                wcnct6: table.get_field("wcnct6").unwrap().to_string(row_index).unwrap(), 
                                wcnct7: table.get_field("wcnct7").unwrap().to_string(row_index).unwrap(), 
                                vl_gname: table.get_field("vl_gname").unwrap().to_string(row_index).unwrap(), 
                                vl_type: table.get_field("vl_type").unwrap().to_u8(row_index).unwrap(), 
                                vl_prio: table.get_field("vl_prio").unwrap().to_u8(row_index).unwrap(), 
                                vl_phtime: table.get_field("vl_phtime").unwrap().to_u16(row_index).unwrap(), 
                                vl_pcdlt: table.get_field("vl_pcdlt").unwrap().to_i8(row_index).unwrap(), 
                                p3d_vo: table.get_field("p3d_vo").unwrap().to_i16(row_index).unwrap(), 
                                p3d_vg: table.get_field("p3d_vg").unwrap().to_i16(row_index).unwrap(), 
                                p3d_ao: table.get_field("p3d_ao").unwrap().to_i16(row_index).unwrap(), 
                                p3d_ag: table.get_field("p3d_ag").unwrap().to_i16(row_index).unwrap(), 
                                p3d_ido: table.get_field("p3d_ido").unwrap().to_i16(row_index).unwrap(), 
                                p3d_idg: table.get_field("p3d_idg").unwrap().to_i16(row_index).unwrap(), 
                                dry0g: table.get_field("dry0g").unwrap().to_u8(row_index).unwrap(), 
                                dry1g: table.get_field("dry1g").unwrap().to_u8(row_index).unwrap(), 
                                dry2g: table.get_field("dry2g").unwrap().to_u8(row_index).unwrap(), 
                                dry3g: table.get_field("dry3g").unwrap().to_u8(row_index).unwrap(), 
                                dry4g: table.get_field("dry4g").unwrap().to_u8(row_index).unwrap(), 
                                dry5g: table.get_field("dry5g").unwrap().to_u8(row_index).unwrap(), 
                                dry6g: table.get_field("dry6g").unwrap().to_u8(row_index).unwrap(), 
                                dry7g: table.get_field("dry7g").unwrap().to_u8(row_index).unwrap(), 
                                wet0g: table.get_field("wet0g").unwrap().to_u8(row_index).unwrap(), 
                                wet1g: table.get_field("wet1g").unwrap().to_u8(row_index).unwrap(), 
                                wet2g: table.get_field("wet2g").unwrap().to_u8(row_index).unwrap(), 
                                wet3g: table.get_field("wet3g").unwrap().to_u8(row_index).unwrap(), 
                                wet4g: table.get_field("wet4g").unwrap().to_u8(row_index).unwrap(), 
                                wet5g: table.get_field("wet5g").unwrap().to_u8(row_index).unwrap(), 
                                wet6g: table.get_field("wet6g").unwrap().to_u8(row_index).unwrap(), 
                                wet7g: table.get_field("wet7g").unwrap().to_u8(row_index).unwrap(), 
                                f1_type: table.get_field("f1_type").unwrap().to_u8(row_index).unwrap(), 
                                f1_cofo: table.get_field("f1_cofo").unwrap().to_u16(row_index).unwrap(), 
                                f1_cofg: table.get_field("f1_cofg").unwrap().to_u16(row_index).unwrap(), 
                                f1_resoo: table.get_field("f1_resoo").unwrap().to_u16(row_index).unwrap(), 
                                f1_resog: table.get_field("f1_resog").unwrap().to_u16(row_index).unwrap(), 
                                f2_type: table.get_field("f2_type").unwrap().to_u8(row_index).unwrap(), 
                                f2_coflo: table.get_field("f2_coflo").unwrap().to_u16(row_index).unwrap(), 
                                f2_coflg: table.get_field("f2_coflg").unwrap().to_u16(row_index).unwrap(), 
                                f2_cofho: table.get_field("f2_cofho").unwrap().to_u16(row_index).unwrap(), 
                                f2_cofhg: table.get_field("f2_cofhg").unwrap().to_u16(row_index).unwrap(), 
                            });
                        }
                    }
                    result
                },
                sound_element: parent_fields.sound_element
                    .map(|sdl_table| get_aax_sound_data(&sdl_table, cpk).unwrap()).unwrap(),
                isaac: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.isaac {
                        for row_index in 0..table.row_count() {
                            result.push(CriCsbAaxIsaac {
                                name: table.get_field("name").unwrap().to_string(row_index).unwrap(), 
                                ptname: table.get_field("ptname").unwrap().to_string(row_index).unwrap(), 
                                ttype: table.get_field("type").unwrap().to_u8(row_index).unwrap(), 
                                grph: CriGraph::from_table(&table.get_field("grph").unwrap().to_table(row_index).unwrap()).unwrap(),
                                rndrng: table.get_field("rndrng").unwrap().to_u8(row_index).unwrap(), 
                            });
                        }
                    }
                    result
                },
                voice_limit_group: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.voice_limit_group {
                        for row_index in 0..table.row_count() {
                            // This is a known special case when fields are defined but have no rows
                            if let Ok(vlgname) = table.get_field("vlgname").unwrap().to_string(row_index)
                            && let Ok(vlgnvoice) = table.get_field("vlgnvoice").unwrap().to_u32(row_index) {
                                result.push(CriCsbAaxVoiceLimitGroup {
                                    vlgname,
                                    vlgnvoice,
                                });
                            }
                        }
                    }
                    result
                },
            }
        );
    }
    
    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let mut parent_table = CriTable {
            name: "TBLCSB".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "ttype".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::ProbablyTable,
                    default_value: None,
                    name: "utf".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut info_table = CriTable {
            name: "TBL_INFO".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "DataFmtVer".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "ExtSize".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut cue_table = CriTable {
            name: "TBLCUE".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "id".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "synth".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "udata".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "flags".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut synth_table = CriTable {
            name: "TBLSYN".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "synname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "syntype".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "cmplxtype".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "lnkname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "issetname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: None,
                    name: "volume".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: None,
                    name: "pitch".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "dlytim".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "s_cntrl".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "eg_dly".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "eg_atk".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "eg_hld".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: None,
                    name: "eg_dcy".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: None,
                    name: "eg_rel".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: None,
                    name: "eg_sus".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "f_type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f_cof1".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f_cof2".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f_reso".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "f_roff".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "dryoname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "mtxrtr".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry0".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry1".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry2".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry3".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry4".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry5".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry6".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "dry7".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wetoname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: None,
                    name: "wet0".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet1".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet2".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet3".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet4".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet5".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet6".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "wet7".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct0".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct1".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct2".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct3".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct4".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct5".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct6".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "wcnct7".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: Some(CriFieldValue::String("<NULL>".to_string())),
                    name: "vl_gname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "vl_type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "vl_prio".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "vl_phtime".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::SByte,
                    default_value: Some(CriFieldValue::SByte(0)),
                    name: "vl_pcdlt".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: None,
                    name: "p3d_vo".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: Some(CriFieldValue::Int16(1000)),
                    name: "p3d_vg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: Some(CriFieldValue::Int16(0)),
                    name: "p3d_ao".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: Some(CriFieldValue::Int16(1000)),
                    name: "p3d_ag".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: None,
                    name: "p3d_ido".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Int16,
                    default_value: Some(CriFieldValue::Int16(1000)),
                    name: "p3d_idg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry0g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry1g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry2g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry3g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry4g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry5g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry6g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "dry7g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet0g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet1g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet2g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet3g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet4g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet5g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet6g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(255)),
                    name: "wet7g".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "f1_type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f1_cofo".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f1_cofg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f1_resoo".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f1_resog".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "f2_type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(0)),
                    name: "f2_coflo".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(1000)),
                    name: "f2_coflg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: None,
                    name: "f2_cofho".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt16,
                    default_value: Some(CriFieldValue::UInt16(1000)),
                    name: "f2_cofhg".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut sound_element_table = CriTable {
            name: "TBLSDL".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Data,
                    default_value: None,
                    name: "data".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "fmt".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "nch".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "stmflg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "sfreq".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "nsmpl".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut isaac_table = CriTable {
            name: "TBLISC".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "ptname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Data,
                    default_value: None,
                    name: "grph".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "rndrng".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut voice_limit_group_table = CriTable {
            name: "TBLVLG".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "vlgname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "vlgnvoice".to_string(),
                    rows: vec![],
                },
            ],
        };
        
        for entry in &self.info {
            info_table.fields[0].rows.push(CriFieldValue::UInt32(entry.data_fmt_version));
            info_table.fields[1].rows.push(CriFieldValue::UInt32(entry.ext_size));
        }

        for entry in &self.cue {
            cue_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            cue_table.fields[1].rows.push(CriFieldValue::UInt32(entry.id));
            cue_table.fields[2].rows.push(CriFieldValue::String(entry.synth.clone()));
            cue_table.fields[3].rows.push(CriFieldValue::String(entry.udata.clone()));
            cue_table.fields[4].rows.push(CriFieldValue::Byte(entry.flags));
        }

        for entry in &self.synth {
            synth_table.fields[0].rows.push(CriFieldValue::String(entry.synname.clone()));
            synth_table.fields[1].rows.push(CriFieldValue::Byte(entry.syntype));
            synth_table.fields[2].rows.push(CriFieldValue::Byte(entry.cmplxtype));
            synth_table.fields[3].rows.push(CriFieldValue::String(entry.lnkname.clone()));
            synth_table.fields[4].rows.push(CriFieldValue::String(entry.issetname.clone()));
            synth_table.fields[5].rows.push(CriFieldValue::Int16(entry.volume));
            synth_table.fields[6].rows.push(CriFieldValue::Int16(entry.pitch));
            synth_table.fields[7].rows.push(CriFieldValue::UInt32(entry.dlytim));
            synth_table.fields[8].rows.push(CriFieldValue::Byte(entry.s_cntrl));
            synth_table.fields[9].rows.push(CriFieldValue::UInt16(entry.eg_dly));
            synth_table.fields[10].rows.push(CriFieldValue::UInt16(entry.eg_atk));
            synth_table.fields[11].rows.push(CriFieldValue::UInt16(entry.eg_hld));
            synth_table.fields[12].rows.push(CriFieldValue::UInt16(entry.eg_dcy));
            synth_table.fields[13].rows.push(CriFieldValue::UInt16(entry.eg_rel));
            synth_table.fields[14].rows.push(CriFieldValue::UInt16(entry.eg_sus));
            synth_table.fields[15].rows.push(CriFieldValue::Byte(entry.f_type));
            synth_table.fields[16].rows.push(CriFieldValue::UInt16(entry.f_cof1));
            synth_table.fields[17].rows.push(CriFieldValue::UInt16(entry.f_cof2));
            synth_table.fields[18].rows.push(CriFieldValue::UInt16(entry.f_reso));
            synth_table.fields[19].rows.push(CriFieldValue::Byte(entry.f_roff));
            synth_table.fields[20].rows.push(CriFieldValue::String(entry.dryoname.clone()));
            synth_table.fields[21].rows.push(CriFieldValue::String(entry.mtxrtr.clone()));
            synth_table.fields[22].rows.push(CriFieldValue::UInt16(entry.dry0));
            synth_table.fields[23].rows.push(CriFieldValue::UInt16(entry.dry1));
            synth_table.fields[24].rows.push(CriFieldValue::UInt16(entry.dry2));
            synth_table.fields[25].rows.push(CriFieldValue::UInt16(entry.dry3));
            synth_table.fields[26].rows.push(CriFieldValue::UInt16(entry.dry4));
            synth_table.fields[27].rows.push(CriFieldValue::UInt16(entry.dry5));
            synth_table.fields[28].rows.push(CriFieldValue::UInt16(entry.dry6));
            synth_table.fields[29].rows.push(CriFieldValue::UInt16(entry.dry7));
            synth_table.fields[30].rows.push(CriFieldValue::String(entry.wetoname.clone()));
            synth_table.fields[31].rows.push(CriFieldValue::UInt16(entry.wet0));
            synth_table.fields[32].rows.push(CriFieldValue::UInt16(entry.wet1));
            synth_table.fields[33].rows.push(CriFieldValue::UInt16(entry.wet2));
            synth_table.fields[34].rows.push(CriFieldValue::UInt16(entry.wet3));
            synth_table.fields[35].rows.push(CriFieldValue::UInt16(entry.wet4));
            synth_table.fields[36].rows.push(CriFieldValue::UInt16(entry.wet5));
            synth_table.fields[37].rows.push(CriFieldValue::UInt16(entry.wet6));
            synth_table.fields[38].rows.push(CriFieldValue::UInt16(entry.wet7));
            synth_table.fields[39].rows.push(CriFieldValue::String(entry.wcnct0.clone()));
            synth_table.fields[40].rows.push(CriFieldValue::String(entry.wcnct1.clone()));
            synth_table.fields[41].rows.push(CriFieldValue::String(entry.wcnct2.clone()));
            synth_table.fields[42].rows.push(CriFieldValue::String(entry.wcnct3.clone()));
            synth_table.fields[43].rows.push(CriFieldValue::String(entry.wcnct4.clone()));
            synth_table.fields[44].rows.push(CriFieldValue::String(entry.wcnct5.clone()));
            synth_table.fields[45].rows.push(CriFieldValue::String(entry.wcnct6.clone()));
            synth_table.fields[46].rows.push(CriFieldValue::String(entry.wcnct7.clone()));
            synth_table.fields[47].rows.push(CriFieldValue::String(entry.vl_gname.clone()));
            synth_table.fields[48].rows.push(CriFieldValue::Byte(entry.vl_type));
            synth_table.fields[49].rows.push(CriFieldValue::Byte(entry.vl_prio));
            synth_table.fields[50].rows.push(CriFieldValue::UInt16(entry.vl_phtime));
            synth_table.fields[51].rows.push(CriFieldValue::SByte(entry.vl_pcdlt));
            synth_table.fields[52].rows.push(CriFieldValue::Int16(entry.p3d_vo));
            synth_table.fields[53].rows.push(CriFieldValue::Int16(entry.p3d_vg));
            synth_table.fields[54].rows.push(CriFieldValue::Int16(entry.p3d_ao));
            synth_table.fields[55].rows.push(CriFieldValue::Int16(entry.p3d_ag));
            synth_table.fields[56].rows.push(CriFieldValue::Int16(entry.p3d_ido));
            synth_table.fields[57].rows.push(CriFieldValue::Int16(entry.p3d_idg));
            synth_table.fields[58].rows.push(CriFieldValue::Byte(entry.dry0g));
            synth_table.fields[59].rows.push(CriFieldValue::Byte(entry.dry1g));
            synth_table.fields[60].rows.push(CriFieldValue::Byte(entry.dry2g));
            synth_table.fields[61].rows.push(CriFieldValue::Byte(entry.dry3g));
            synth_table.fields[62].rows.push(CriFieldValue::Byte(entry.dry4g));
            synth_table.fields[63].rows.push(CriFieldValue::Byte(entry.dry5g));
            synth_table.fields[64].rows.push(CriFieldValue::Byte(entry.dry6g));
            synth_table.fields[65].rows.push(CriFieldValue::Byte(entry.dry7g));
            synth_table.fields[66].rows.push(CriFieldValue::Byte(entry.wet0g));
            synth_table.fields[67].rows.push(CriFieldValue::Byte(entry.wet1g));
            synth_table.fields[68].rows.push(CriFieldValue::Byte(entry.wet2g));
            synth_table.fields[69].rows.push(CriFieldValue::Byte(entry.wet3g));
            synth_table.fields[70].rows.push(CriFieldValue::Byte(entry.wet4g));
            synth_table.fields[71].rows.push(CriFieldValue::Byte(entry.wet5g));
            synth_table.fields[72].rows.push(CriFieldValue::Byte(entry.wet6g));
            synth_table.fields[73].rows.push(CriFieldValue::Byte(entry.wet7g));
            synth_table.fields[74].rows.push(CriFieldValue::Byte(entry.f1_type));
            synth_table.fields[75].rows.push(CriFieldValue::UInt16(entry.f1_cofo));
            synth_table.fields[76].rows.push(CriFieldValue::UInt16(entry.f1_cofg));
            synth_table.fields[77].rows.push(CriFieldValue::UInt16(entry.f1_resoo));
            synth_table.fields[78].rows.push(CriFieldValue::UInt16(entry.f1_resog));
            synth_table.fields[79].rows.push(CriFieldValue::Byte(entry.f2_type));
            synth_table.fields[80].rows.push(CriFieldValue::UInt16(entry.f2_coflo));
            synth_table.fields[81].rows.push(CriFieldValue::UInt16(entry.f2_coflg));
            synth_table.fields[82].rows.push(CriFieldValue::UInt16(entry.f2_cofho));
            synth_table.fields[83].rows.push(CriFieldValue::UInt16(entry.f2_cofhg));
        }

        for entry in &self.sound_element {
            sound_element_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            sound_element_table.fields[1].rows.push(CriFieldValue::Data(match &entry.data {
                CriCsbAaxSoundElementData::Aax(cri_aax_archive) => cri_aax_archive
                    .write().unwrap(),
                CriCsbAaxSoundElementData::Stream(_) => {
                    todo!("cpk stream write");
                }
            }));
            // sound_element_table.fields[2].rows.push(CriFieldValue::Byte(entry.fmt));
            sound_element_table.fields[3].rows.push(CriFieldValue::Byte(entry.nch));
            // sound_element_table.fields[4].rows.push(CriFieldValue::Byte(entry.stmflg));
            sound_element_table.fields[5].rows.push(CriFieldValue::UInt32(entry.sfreq));
            sound_element_table.fields[6].rows.push(CriFieldValue::UInt32(entry.nsmpl));
        }

        for entry in &self.isaac {
            isaac_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            isaac_table.fields[1].rows.push(CriFieldValue::String(entry.ptname.clone()));
            isaac_table.fields[2].rows.push(CriFieldValue::Byte(entry.ttype));
            isaac_table.fields[3].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(entry.grph.to_table().unwrap())));
            isaac_table.fields[4].rows.push(CriFieldValue::Byte(entry.rndrng));
        }

        for entry in &self.voice_limit_group {
            voice_limit_group_table.fields[0].rows.push(CriFieldValue::String(entry.vlgname.clone()));
            voice_limit_group_table.fields[1].rows.push(CriFieldValue::UInt32(entry.vlgnvoice.clone()));
        }

        parent_table.fields[0].rows.push(CriFieldValue::String("INFO".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(7));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(info_table)));

        parent_table.fields[0].rows.push(CriFieldValue::String("CUE".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(1));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(cue_table)));

        parent_table.fields[0].rows.push(CriFieldValue::String("SYNTH".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(2));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(synth_table)));
        
        parent_table.fields[0].rows.push(CriFieldValue::String("SOUND_ELEMENT".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(4));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(sound_element_table)));
        
        parent_table.fields[0].rows.push(CriFieldValue::String("ISAAC".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(5));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(isaac_table)));
        
        parent_table.fields[0].rows.push(CriFieldValue::String("VOICE_LIMIT_GROUP".to_string()));
        parent_table.fields[1].rows.push(CriFieldValue::Byte(6));
        parent_table.fields[2].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(voice_limit_group_table)));

        parent_table.write()
    }
}