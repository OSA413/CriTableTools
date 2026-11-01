use cri_table::table::CriTable;

use crate::{csb::{common::{cue::CriCsbCue, info::CriCsbInfo, isaac::CriCsbIsaac, synth::CriCsbSynth, voice_limit_group::CriCsbVoiceLimitGroup}, sound_element::CriCsbSoundElement}, graph::CriGraphWrapper, sound_archive::CriCsbSoundArchiveEnum, sound_archive_mode::CriSoundArchiveFormat, traits::FromCriTable};

pub struct CriCsbTablesBuilder {
    pub info: Option<CriTable>,
    pub cue: Option<CriTable>,
    pub synth: Option<CriTable>,
    pub sound_element: Option<CriTable>,
    pub isaac: Option<CriTable>,
    pub voice_limit_group: Option<CriTable>,
}

impl CriCsbTablesBuilder {
    pub fn parse_info_table(table: Option<&CriTable>) -> (Vec<CriCsbInfo>, Vec<String>) {
        if let Some(table) = table {                        
            let mut result = Vec::with_capacity(table.row_count());
            for row_index in 0..table.row_count() {
                result.push(CriCsbInfo {
                    original_string_pool: table.original_string_pool.clone(),
                    data_fmt_version: table.get_field("DataFmtVer").unwrap().to_u32(row_index).unwrap(),
                    ext_size: table.get_field("ExtSize").unwrap().to_u32(row_index).unwrap(),
                });
            }
            return (result, table.original_string_pool.clone());
        }
        (vec![], vec![])
    }

    pub fn parse_cue_table(table: Option<&CriTable>) -> (Vec<CriCsbCue>, Vec<String>) {
        if let Some(table) = table {
            let mut result = Vec::with_capacity(table.row_count());
            for row_index in 0..table.row_count() {
                result.push(CriCsbCue {
                    original_string_pool: table.original_string_pool.clone(),
                    name: table.get_field("name").unwrap().to_string(row_index).unwrap(),
                    id: table.get_field("id").unwrap().to_u32(row_index).unwrap(),
                    synth: table.get_field("synth").unwrap().to_string(row_index).unwrap(),
                    udata: table.get_field("udata").unwrap().to_string(row_index).unwrap(),
                    flags: table.get_field("flags").unwrap().to_u8(row_index).unwrap(),
                });
            }
            return (result, table.original_string_pool.clone());
        }
        (vec![], vec![])
    }

    pub fn parse_synth_table(table: Option<&CriTable>) -> (Vec<CriCsbSynth>, Vec<String>) {
        if let Some(table) = table {
            let mut result = Vec::with_capacity(table.row_count());
            for row_index in 0..table.row_count() {
                result.push(CriCsbSynth {
                    original_string_pool: table.original_string_pool.clone(),
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
            return (result, table.original_string_pool.clone())
        }
        (vec![], vec![])
    }
    
    pub fn parse_sound_element_table(table: Option<&CriTable>) -> (Vec<CriCsbSoundElement>, Vec<String>) {
        if let Some(table) = table {
            let format = table.get_field("fmt").unwrap().to_u8(0).unwrap();
            let format = match format {
                0 => CriSoundArchiveFormat::Adx,
                4 => CriSoundArchiveFormat::Dsp,
                // 5 => CriSoundArchiveFormat::Wav,
                _ => panic!("The given CSB file contains an audio file which is not of supported format. Only CSB files with ADX or DSP files are supported."),
            };

            let mut result = Vec::with_capacity(table.row_count());
            for sdl_row_index in 0..table.row_count() {
                let sdl_name = table.get_field("name").unwrap().to_string(sdl_row_index).unwrap();                
                let sound_element_table = table.get_field("data").unwrap().to_table(sdl_row_index).unwrap();
                let data = CriCsbSoundArchiveEnum::from_table(sound_element_table).unwrap();

                match (&format, &data) {
                    (CriSoundArchiveFormat::Adx, CriCsbSoundArchiveEnum::Aax(_)) => (),
                    (CriSoundArchiveFormat::Dsp, CriCsbSoundArchiveEnum::AdpcmWii(_)) => (),
                    (format, data) =>
                        panic!("Incorret combination of foemat and data: {format:?}, {data:?}"),
                }

                result.push(CriCsbSoundElement {
                    original_string_pool: sound_element_table.original_string_pool.clone(),
                    name: sdl_name.clone(),
                    data,
                    fmt: table.get_field("fmt").unwrap().to_u8(sdl_row_index).unwrap(),
                    nch: table.get_field("nch").unwrap().to_u8(sdl_row_index).unwrap(),
                    stmflg: table.get_field("stmflg").unwrap().to_u8(sdl_row_index).unwrap(),
                    sfreq: table.get_field("sfreq").unwrap().to_u32(sdl_row_index).unwrap(),
                    nsmpl: table.get_field("nsmpl").unwrap().to_u32(sdl_row_index).unwrap(),
                });
            }
            return (result, table.original_string_pool.clone());
        }
        (vec![], vec![])
    }

    pub fn parse_isaac_table(table: Option<&CriTable>) -> (Vec<CriCsbIsaac>, Vec<String>) {
        let mut result = Vec::new();
        if let Some(table) = table {
            for row_index in 0..table.row_count() {
                result.push(CriCsbIsaac {
                    original_string_pool: table.original_string_pool.clone(),
                    name: table.get_field("name").unwrap().to_string(row_index).unwrap(), 
                    ptname: table.get_field("ptname").unwrap().to_string(row_index).unwrap(), 
                    ttype: table.get_field("type").unwrap().to_u8(row_index).unwrap(), 
                    grph: CriGraphWrapper::from_table(table.get_field("grph").unwrap().to_table(row_index).unwrap()).unwrap(),
                    rndrng: table.get_field("rndrng").unwrap().to_u8(row_index).unwrap(), 
                });
            }
            return (result, table.original_string_pool.clone());
        }
        (vec![], vec![])
    }

    pub fn parse_voice_group_table(table: Option<&CriTable>) -> (Vec<CriCsbVoiceLimitGroup>, Vec<String>) {
        let mut result = Vec::new();
        if let Some(table) = table && table.row_count() != 0 {
            for row_index in 0..table.row_count() {
                // This is a known special case when fields are defined but have no rows
                if let Some(vlgname_field) = table.get_field("vlgname")
                && let Ok(vlgname) = vlgname_field.to_string(row_index)
                && let Some(vlgnvoice_field) = table.get_field("vlgnvoice")
                && let Ok(vlgnvoice) = vlgnvoice_field.to_u32(row_index) {
                    result.push(CriCsbVoiceLimitGroup {
                        original_string_pool: table.original_string_pool.clone(),
                        vlgname,
                        vlgnvoice,
                    });
                }
            }
            return (result, table.original_string_pool.clone());
        }
        (vec![], vec![])
    }

    pub fn from_table(csb_table: &CriTable) -> CriCsbTablesBuilder {
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
}
