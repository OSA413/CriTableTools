use common_binary::error::CommonBinaryError;
use cri_table::{field::CriTableField, field_flags::CriFieldFlags, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, encoding_type::CriTableEncodingType, table::CriTable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriCsbAdpcmWiiInfo{ 
    pub data_fmt_version: u32,
    pub ext_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbAdpcmWiiCue{ 
    pub name: String,
    pub id: u32,
    pub synth: String,
    pub udata: String,
    pub flags: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CriCsbAdpcmWiiSynth{ 
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


#[derive(Serialize, Deserialize)]
pub struct CriCsbAdpcmWiiVoiceLimitGroup{ 
    pub vlgname: String,
    pub vlgnvoice: u32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CriCsbAdpcmWiiIsaac {
    pub name: String,
    pub ptname: String,
    pub ttype: u8,
    pub grph: CriGraphWrapper,
    pub rndrng: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CriGraphWrapper {
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

    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut table = CriTable {
            name: "TBLIPT".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    flags: CriFieldFlags {
                        row_storage: true,
                        name: true,
                        default_value: false,
                    },
                    ttype: CriFieldType::UInt16,
                    name: "in".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    flags: CriFieldFlags {
                        row_storage: true,
                        name: true,
                        default_value: false,
                    },
                    ttype: CriFieldType::UInt16,
                    name: "out".to_string(),
                    rows: vec![],
                    default_value: None,
                }
            ],
        };
        
        for entry in &self.entries {
            table.fields[0].rows.push(CriFieldValue::UInt16(entry.in_));
            table.fields[1].rows.push(CriFieldValue::UInt16(entry.out));
        }

        Ok(table)
    }
}

impl CriGraphWrapper {
    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut table = CriTable {
            name: "TBLIGR".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
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

impl CriGraph {
    pub fn from_table(table: &CriTable) -> Result<CriGraphWrapper, CommonBinaryError> {
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
            entries,
        })
    }
}
