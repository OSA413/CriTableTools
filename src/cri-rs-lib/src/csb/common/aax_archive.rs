use std::{fs, path::Path};

use common_binary::error::CommonBinaryError;
use cri_table::{field_value::{CriFieldValue, TableOrData}, table::{CriTable, UTF_SIGNATURE}};

use crate::{cpk::sound::archive::CriCpkArchive, csb::common::{aax_sound_element::{CriCsbAaxSoundElement}, archive::CriCsbTablesBuilder, cue::CriCsbCue, info::CriCsbInfo, isaac::CriCsbIsaac, synth::CriCsbSynth, voice_limit_group::CriCsbVoiceLimitGroup}, find_related_cpk_of_csb, traits::ToCriTable};

pub struct CriCsbAaxArchive {
    pub original_string_pool: Vec<String>,
    pub info: (Vec<CriCsbInfo>, Vec<String>),
    pub cue: (Vec<CriCsbCue>, Vec<String>),
    pub synth: (Vec<CriCsbSynth>, Vec<String>),
    pub sound_element: (Vec<CriCsbAaxSoundElement>, Vec<String>),
    pub isaac: (Vec<CriCsbIsaac>, Vec<String>),
    pub voice_limit_group: (Vec<CriCsbVoiceLimitGroup>, Vec<String>),
}

impl CriCsbAaxArchive {
    pub fn read(path: &Path) -> Result<CriCsbAaxArchive, CommonBinaryError> {
        let file_content = fs::read(path).unwrap();
        let csb_table = CriTable::read(&file_content, Some(0), UTF_SIGNATURE).unwrap();

        let cpk_path = find_related_cpk_of_csb(path);
        let cpk_data = cpk_path.map(|path| fs::read(path).unwrap());
        // We no longer read CPK when reading CSB
        let _cpk = cpk_data.map(|data| CriCpkArchive::read(&data, 0).unwrap());

        let parent_fields = CriCsbTablesBuilder::from_table(&csb_table);

        Ok(
            CriCsbAaxArchive {
                original_string_pool: csb_table.original_string_pool,
                info: CriCsbTablesBuilder::parse_info_table(parent_fields.info.as_ref()),
                cue: CriCsbTablesBuilder::parse_cue_table(parent_fields.cue.as_ref()),
                synth: CriCsbTablesBuilder::parse_synth_table(parent_fields.synth.as_ref()),
                sound_element: CriCsbTablesBuilder::parse_sound_element_table_aax(parent_fields.sound_element.as_ref()),
                isaac: CriCsbTablesBuilder::parse_isaac_table(parent_fields.isaac.as_ref()),
                voice_limit_group: CriCsbTablesBuilder::parse_voice_group_table(parent_fields.sound_element.as_ref()),
            }
        )
    }
    
    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let mut parent_table = CriTable::new_csb(&self.original_string_pool, 6);
        let mut info_table = CriTable::new_info(&self.info.1, self.info.0.len());
        let mut cue_table = CriTable::new_cue(&self.cue.1, self.cue.0.len());
        let mut synth_table = CriTable::new_synth(&self.synth.1, self.synth.0.len());
        let mut sound_element_table = CriTable::new_sound_element(&self.sound_element.1, self.sound_element.0.len());
        let mut isaac_table = CriTable::new_isaac(&self.isaac.1, self.isaac.0.len());
        let mut voice_limit_group_table = CriTable::new_voice_limit_group(&self.voice_limit_group.1, self.voice_limit_group.0.len());
        
        for entry in &self.info.0 {
            info_table.fields[0].rows.push(CriFieldValue::UInt32(entry.data_fmt_version));
            info_table.fields[1].rows.push(CriFieldValue::UInt32(entry.ext_size));
        }

        for entry in &self.cue.0 {
            cue_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            cue_table.fields[1].rows.push(CriFieldValue::UInt32(entry.id));
            cue_table.fields[2].rows.push(CriFieldValue::String(entry.synth.clone()));
            cue_table.fields[3].rows.push(CriFieldValue::String(entry.udata.clone()));
            cue_table.fields[4].rows.push(CriFieldValue::Byte(entry.flags));
        }

        for entry in &self.synth.0 {
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

        for entry in &self.sound_element.0 {
            sound_element_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            // TODO REDO NOTE THIS IS WRONG
            sound_element_table.fields[1].rows.push(CriFieldValue::Data(entry.data.entries.first().unwrap().data.clone()));
            sound_element_table.fields[2].rows.push(CriFieldValue::Byte(entry.fmt));
            sound_element_table.fields[3].rows.push(CriFieldValue::Byte(entry.nch));
            sound_element_table.fields[4].rows.push(CriFieldValue::Byte(entry.stmflg));
            sound_element_table.fields[5].rows.push(CriFieldValue::UInt32(entry.sfreq));
            sound_element_table.fields[6].rows.push(CriFieldValue::UInt32(entry.nsmpl));
        }

        for entry in &self.isaac.0 {
            isaac_table.fields[0].rows.push(CriFieldValue::String(entry.name.clone()));
            isaac_table.fields[1].rows.push(CriFieldValue::String(entry.ptname.clone()));
            isaac_table.fields[2].rows.push(CriFieldValue::Byte(entry.ttype));
            isaac_table.fields[3].rows.push(CriFieldValue::ProbablyTable(TableOrData::Table(entry.grph.to_table().unwrap())));
            isaac_table.fields[4].rows.push(CriFieldValue::Byte(entry.rndrng));
        }

        for entry in &self.voice_limit_group.0 {
            voice_limit_group_table.fields[0].rows.push(CriFieldValue::String(entry.vlgname.clone()));
            voice_limit_group_table.fields[1].rows.push(CriFieldValue::UInt32(entry.vlgnvoice));
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