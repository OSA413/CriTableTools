use std::{fs, path::Path};

use common_binary::error::CommonBinaryError;
use cri_table::{field::CriTableField, field_flags::CriFieldFlags, field_type::CriFieldType, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType, table::{CriTable, UTF_SIGNATURE}};
use serde::{Deserialize, Serialize};
use crate::{adpcm_wii::archive::{CriAdpcmWiiArchive, CriAdpcmWiiArchiveWrapper}, cpk::sound::archive::CriCpkArchive};

struct CriCsbTables {
    pub info: Option<CriTable>,
    pub cue: Option<CriTable>,
    pub synth: Option<CriTable>,
    pub sound_element: Option<CriTable>,
    pub isaac: Option<CriTable>,
    pub voice_limit_group: Option<CriTable>,
}

pub struct CriCsbArchive {
    pub info: Vec<CriCsbInfo>,
    pub cue: Vec<CriCsbCue>,
    pub synth: Vec<CriCsbSynth>,
    pub sound_element: Vec<CriCsbSoundElement>,
    pub isaac: Vec<CriCsbIsaac>,
    pub voice_limit_group: Vec<CriCsbVoiceLimitGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbInfo{ 
    pub data_fmt_version: u32,
    pub ext_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbCue{ 
    pub name: String,
    pub id: u32,
    pub synth: String,
    pub udata: String,
    pub flags: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbSynth{ 
    pub synname: String,
    pub syntype: u8,
    pub cmplxtype: u8,
    pub lnkname: String,
    pub issetname: String,
    pub volume: i16,
    pub pitch: i16,
    pub dlytim: u32,
    pub s_cntrl: u8,
    pub eg_dly: u16,
    pub eg_atk: u16,
    pub eg_hld: u16,
    pub eg_dcy: u16,
    pub eg_rel: u16,
    pub eg_sus: u16,
    pub f_type: u8,
    pub f_cof1: u16,
    pub f_cof2: u16,
    pub f_reso: u16,
    pub f_roff: u8,
    pub dryoname: String,
    pub mtxrtr: String,
    // Wet-dry world
    pub dry0: u16,
    pub dry1: u16,
    pub dry2: u16,
    pub dry3: u16,
    pub dry4: u16,
    pub dry5: u16,
    pub dry6: u16,
    pub dry7: u16,
    pub wetoname: String,
    pub wet0: u16,
    pub wet1: u16,
    pub wet2: u16,
    pub wet3: u16,
    pub wet4: u16,
    pub wet5: u16,
    pub wet6: u16,
    pub wet7: u16,
    pub wcnct0: String,
    pub wcnct1: String,
    pub wcnct2: String,
    pub wcnct3: String,
    pub wcnct4: String,
    pub wcnct5: String,
    pub wcnct6: String,
    pub wcnct7: String,
    pub vl_gname: String,
    pub vl_type: u8,
    pub vl_prio: u8,
    pub vl_phtime: u16,
    pub vl_pcdlt: i8,
    pub p3d_vo: i16,
    pub p3d_vg: i16,
    pub p3d_ao: i16,
    pub p3d_ag: i16,
    pub p3d_ido: i16,
    pub p3d_idg: i16,
    // Wet-dry world, again
    pub dry0g: u8,
    pub dry1g: u8,
    pub dry2g: u8,
    pub dry3g: u8,
    pub dry4g: u8,
    pub dry5g: u8,
    pub dry6g: u8,
    pub dry7g: u8,
    pub wet0g: u8,
    pub wet1g: u8,
    pub wet2g: u8,
    pub wet3g: u8,
    pub wet4g: u8,
    pub wet5g: u8,
    pub wet6g: u8,
    pub wet7g: u8,
    pub f1_type: u8,
    pub f1_cofo: u16,
    pub f1_cofg: u16,
    pub f1_resoo: u16,
    pub f1_resog: u16,
    pub f2_type: u8,
    pub f2_coflo: u16,
    pub f2_coflg: u16,
    pub f2_cofho: u16,
    pub f2_cofhg: u16,
}

pub struct CriCsbSoundElement {
    pub name: String,
    pub data: CriCsbAdpcmWiiSoundElementData,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbSoundElementMetaInfo {
    pub name: String,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}

#[derive(Debug)]
pub enum CriCsbAdpcmWiiSoundElementData {
    AdpcmWii(CriAdpcmWiiArchiveWrapper),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CriCsbIsaac {
    pub name: String,
    pub ptname: String,
    pub ttype: u8,
    pub grph: CriGrapthWrapper,
    pub rndrng: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGrapthWrapper {
    entries: Vec<CriGraph>,
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
    pub entries: Vec<CriGraphPointsEntry>,
}

impl CriGraphPoints {
    pub fn from_table(table: &CriTable) -> Result<CriGraphPoints, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());

        for row_index in 0..table.row_count() {
            entries.push(CriGraphPointsEntry {
                in_: table.get_field("in").unwrap().to_u16(row_index).unwrap(),
                out: table.get_field("out").unwrap().to_u16(row_index).unwrap(),
            });
        }

        Ok(CriGraphPoints {
            entries
        })
    }
}


impl CriGraph {
    pub fn from_table(table: &CriTable) -> Result<CriGrapthWrapper, CommonBinaryError> {
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

        Ok(CriGrapthWrapper {
            entries,
        })
    }

    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut table = CriTable {
            name: "TBLIGR".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    name: "type".to_string(),
                    rows: vec![],
                    default_value: None
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Single,
                    name: "imax".to_string(),
                    rows: vec![],
                    default_value: None
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Single,
                    name: "imin".to_string(),
                    rows: vec![],
                    default_value: None
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Single,
                    name: "omax".to_string(),
                    rows: vec![],
                    default_value: None
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Single,
                    name: "omin".to_string(),
                    rows: vec![],
                    default_value: None
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::ProbablyTable,
                    name: "points".to_string(),
                    rows: vec![],
                    default_value: None
                }
            ],
            unknown_byte: 0,
        };



        Ok(table)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbVoiceLimitGroup{ 
    pub vlgname: String,
    pub vlgnvoice: u32,
}

fn get_sound_data(sdl_table: &CriTable) -> Result<Vec<CriCsbSoundElement>, CommonBinaryError> {
    // from_table expects the TBLSDL table (not the inner ADPCM_WII table)
    let wrapper = CriAdpcmWiiArchiveWrapper::from_table(sdl_table).unwrap();
    let mut sound_entries = Vec::with_capacity(wrapper.entries.len());
    for archive in wrapper.entries {
        let CriAdpcmWiiArchive { fmt, name, nch, nsmpl, sfreq, stmflg, entries } = archive;
        sound_entries.push(CriCsbSoundElement {
            name: name.clone(),
            data: CriCsbAdpcmWiiSoundElementData::AdpcmWii(
                CriAdpcmWiiArchiveWrapper { entries: vec![CriAdpcmWiiArchive { fmt, name, nch, nsmpl, sfreq, stmflg, entries }] }
            ),
            fmt,
            nch,
            stmflg,
            sfreq,
            nsmpl,
        });
    }
    Ok(sound_entries)
}

fn get_csb_fields(csb_table: &CriTable) -> CriCsbTables {
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

    CriCsbTables {
        info,
        cue,
        synth,
        sound_element,
        isaac,
        voice_limit_group,
    }
}

impl CriCsbArchive {
    pub fn read(path: &Path) -> Result<CriCsbArchive, CommonBinaryError> {
        let file_content = fs::read(path).unwrap();
        let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();

        let parent_fields = get_csb_fields(&csb_table);

        return Ok(
            CriCsbArchive {
                info: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.info {
                        let row_count = match table.fields.first() {
                            Some(field) => field.rows.len(),
                            None => 0,
                        };
                        
                        for row_index in 0..row_count {
                            result.push(CriCsbInfo {
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
                        let row_count = match table.fields.first() {
                            Some(field) => field.rows.len(),
                            None => 0,
                        };
                        
                        for row_index in 0..row_count {
                            result.push(CriCsbCue {
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
                        let row_count = match table.fields.first() {
                            Some(field) => field.rows.len(),
                            None => 0,
                        };
                        
                        for row_index in 0..row_count {
                            result.push(CriCsbSynth {
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
                    .map(|sdl_table| get_sound_data(&sdl_table).unwrap()).unwrap(),
                isaac: {
                    let mut result = Vec::new();
                    if let Some(table) = parent_fields.isaac {
                        let row_count = match table.fields.first() {
                            Some(field) => field.rows.len(),
                            None => 0,
                        };
                        
                        for row_index in 0..row_count {
                            result.push(CriCsbIsaac {
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
                        let row_count = match table.fields.first() {
                            Some(field) => field.rows.len(),
                            None => 0,
                        };

                        for row_index in 0..row_count {
                            // This is a known special case when fields are defined but have no rows
                            if let Ok(vlgname) = table.get_field("vlgname").unwrap().to_string(row_index)
                            && let Ok(vlgnvoice) = table.get_field("vlgnvoice").unwrap().to_u32(row_index) {
                                result.push(CriCsbVoiceLimitGroup {
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
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "ttype".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::ProbablyTable,
                    default_value: None,
                    name: "utf".to_string(),
                    rows: vec![],
                },
            ],
        };

        let mut info_table = CriTable {
            name: "TBL_INFO".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "DataFmtVer".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "ExtSize".to_string(),
                    rows: vec![],
                },
            ],
        };
        let mut cue_table = CriTable {
            name: "TBLCUE".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "id".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "synth".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "udata".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "flags".to_string(),
                    rows: vec![],
                },
            ],
        };
        let mut synth_table = CriTable {
            name: "TBLSYN".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "cmplxtype".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "synname".to_string(),
                    ttype: CriFieldType::String,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "syntype".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "cmplxtype".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "lnkname".to_string(),
                    ttype: CriFieldType::String,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "issetname".to_string(),
                    ttype: CriFieldType::String,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "volume".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "pitch".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "dlytim".to_string(),
                    ttype: CriFieldType::UInt32,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "s_cntrl".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: None,
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "eg_dly".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "eg_atk".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "eg_hld".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "eg_dcy".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "eg_rel".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "eg_sus".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f_type".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "vl_prio".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,  
                    },
                    default_value: Some(CriFieldValue::UInt16(0)),
                    rows: vec![],
                    name: "vl_phtime".to_string(),
                    ttype: CriFieldType::UInt16
                },
                CriTableField {
                    default_value: Some(CriFieldValue::SByte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "vl_pcdlt".to_string(),
                    ttype: CriFieldType::SByte,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "p3d_vo".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Int16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "p3d_vg".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Int16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "p3d_ao".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Int16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "p3d_ag".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "p3d_ido".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Int16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "p3d_idg".to_string(),
                    ttype: CriFieldType::Int16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry0g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry1g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry2g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry3g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry4g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry5g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry6g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "dry7g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet0g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet1g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet2g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet3g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet4g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet5g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet6g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "wet7g".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f1_type".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f1_cof".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,  
                    },
                    default_value: Some(CriFieldValue::UInt16(0)),
                    rows: vec![],
                    name: "f1_cofg".to_string(),
                    ttype: CriFieldType::UInt16
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f1_res".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,  
                    },
                    default_value: Some(CriFieldValue::UInt16(0)),
                    rows: vec![],
                    name: "f1_resog".to_string(),
                    ttype: CriFieldType::UInt16
                },
                CriTableField {
                    default_value: Some(CriFieldValue::Byte(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f2_type".to_string(),
                    ttype: CriFieldType::Byte,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f2_cof".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f2_cof".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    rows: vec![],
                    default_value: None,
                    name: "f2_cofho".to_string(),
                    ttype: CriFieldType::UInt16,
                },
                CriTableField {
                    default_value: Some(CriFieldValue::UInt16(0)),
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    rows: vec![],
                    name: "f2_cof".to_string(),
                ttype: CriFieldType::UInt16,
                }
            ],
        };
        let mut sound_element_table = CriTable {
            name: "TBLSDL".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Data,
                    default_value: None,
                    name: "data".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(4)),
                    name: "fmt".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "nch".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: true,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: Some(CriFieldValue::Byte(0)),
                    name: "stmflg".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "sfreq".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::UInt32,
                    default_value: None,
                    name: "nsmpl".to_string(),
                    rows: vec![],
                },
            ],
        };
        let mut isaac_table = CriTable {
            name: "TBLISC".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "name".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "type".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Data,
                    default_value: None,
                    name: "grph".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::Byte,
                    default_value: None,
                    name: "rndrng".to_string(),
                    rows: vec![],
                },
            ],
        };
        let mut voice_limit_group_table = CriTable {
            name: "TBLVLG".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
                    ttype: CriFieldType::String,
                    default_value: None,
                    name: "vlgname".to_string(),
                    rows: vec![],
                },
                CriTableField {
                    flags: CriFieldFlags {
                        name: true,
                        default_value: false,
                        row_storage: true,
                    },
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
                CriCsbAdpcmWiiSoundElementData::AdpcmWii(cri_aax_archive) => cri_aax_archive.to_table().unwrap().write().unwrap(),
            }));
            sound_element_table.fields[2].rows.push(CriFieldValue::Byte(entry.fmt));
            sound_element_table.fields[3].rows.push(CriFieldValue::Byte(entry.nch));
            sound_element_table.fields[4].rows.push(CriFieldValue::Byte(entry.stmflg));
            sound_element_table.fields[5].rows.push(CriFieldValue::UInt32(entry.sfreq));
            sound_element_table.fields[6].rows.push(CriFieldValue::UInt32(entry.nsmpl));
        }

        parent_table.write()
    }
}