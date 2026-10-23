use std::{fs::{self, create_dir_all}, path::Path};

use cri_rs_lib::{adpcm_wii::archive::CriAdpcmWiiArchiveWrapper, csb::games::black_knight::archive::{CriCsbAdpcmWiiSoundElementData, CriCsbArchive, CriCsbCue, CriCsbInfo, CriCsbIsaac, CriCsbSoundElementMetaInfo, CriCsbSynth}};

fn extract_dps(dps: &CriAdpcmWiiArchiveWrapper, base_dir: &Path) {
    for entry in &dps.entries {
        for inner_entry in &entry.entries {
            let entry_dir_name = &base_dir.join(&entry.name);
            create_dir_all(entry_dir_name).unwrap();
            fs::write(
                entry_dir_name.join(inner_entry.loop_flag.to_file_name()),
                [
                    inner_entry.header.clone(),
                    inner_entry.data.clone(),
                ].concat(),
            ).unwrap();

            let raw_root = entry_dir_name.join(Path::new(inner_entry.loop_flag.to_file_name()).parent().unwrap());
            create_dir_all(raw_root.join("raw")).unwrap();
            fs::write(
                raw_root.join("raw/header"),
                &inner_entry.header,
            ).unwrap();
            
            create_dir_all(entry_dir_name).unwrap();
            fs::write(
                raw_root.join("raw/data"),
                &inner_entry.data,
            ).unwrap();

            fs::write(
                entry_dir_name.join("sound_element.json"),
                serde_json::to_string(&CriCsbSoundElementMetaInfo {
                    name: entry.name.clone(),
                    fmt: entry.fmt,
                    nch: entry.nch,
                    stmflg: entry.stmflg,
                    sfreq: entry.sfreq,
                    nsmpl: entry.nsmpl,
                }).unwrap(),
            ).unwrap();
        }
    }
}

pub fn extract_csb(path: &Path) {
    let csb = CriCsbArchive::read(path).unwrap();
    let base_directory = path.parent().unwrap();
    let output_directory_name = base_directory.join(path.file_stem().unwrap());

    for (i, sound_entry) in csb.sound_element.iter().enumerate() {
        match &sound_entry.data {
            CriCsbAdpcmWiiSoundElementData::AdpcmWii(dsp) => extract_dps(&dsp, &output_directory_name),
        };

        let cue = &csb.cue[i];
        fs::write(
            output_directory_name.join(&sound_entry.name).join("cue.json"),
            serde_json::to_string(&CriCsbCue {
                name: cue.name.clone(),
                id: cue.id,
                synth: cue.synth.clone(),
                udata: cue.udata.clone(),
                flags: cue.flags.clone(),
            }).unwrap(),
        ).unwrap();

        let synth = &csb.synth[i];
        fs::write(
            output_directory_name.join(&sound_entry.name).join("synth.json"),
            serde_json::to_string(&CriCsbSynth {
                synname: synth.synname.clone(),
                syntype: synth.syntype,
                cmplxtype: synth.cmplxtype,
                lnkname: synth.lnkname.clone(),
                issetname: synth.issetname.clone(),
                volume: synth.volume,
                pitch: synth.pitch,
                dlytim: synth.dlytim,
                s_cntrl: synth.s_cntrl,
                eg_dly: synth.eg_dly,
                eg_atk: synth.eg_atk,
                eg_hld: synth.eg_hld,
                eg_dcy: synth.eg_dcy,
                eg_rel: synth.eg_rel,
                eg_sus: synth.eg_sus,
                f_type: synth.f_type,
                f_cof1: synth.f_cof1,
                f_cof2: synth.f_cof2,
                f_reso: synth.f_reso,
                f_roff: synth.f_roff,
                dryoname: synth.dryoname.clone(),
                mtxrtr: synth.mtxrtr.clone(),
                dry0: synth.dry0,
                dry1: synth.dry1,
                dry2: synth.dry2,
                dry3: synth.dry3,
                dry4: synth.dry4,
                dry5: synth.dry5,
                dry6: synth.dry6,
                dry7: synth.dry7,
                wetoname: synth.wetoname.clone(),
                wet0: synth.wet0,
                wet1: synth.wet1,
                wet2: synth.wet2,
                wet3: synth.wet3,
                wet4: synth.wet4,
                wet5: synth.wet5,
                wet6: synth.wet6,
                wet7: synth.wet7,
                wcnct0: synth.wcnct0.clone(),
                wcnct1: synth.wcnct1.clone(),
                wcnct2: synth.wcnct2.clone(),
                wcnct3: synth.wcnct3.clone(),
                wcnct4: synth.wcnct4.clone(),
                wcnct5: synth.wcnct5.clone(),
                wcnct6: synth.wcnct6.clone(),
                wcnct7: synth.wcnct7.clone(),
                vl_gname: synth.vl_gname.clone(),
                vl_type: synth.vl_type,
                vl_prio: synth.vl_prio,
                vl_phtime: synth.vl_phtime,
                vl_pcdlt: synth.vl_pcdlt,
                p3d_vo: synth.p3d_vo,
                p3d_vg: synth.p3d_vg,
                p3d_ao: synth.p3d_ao,
                p3d_ag: synth.p3d_ag,
                p3d_ido: synth.p3d_ido,
                p3d_idg: synth.p3d_idg,
                dry0g: synth.dry0g,
                dry1g: synth.dry1g,
                dry2g: synth.dry2g,
                dry3g: synth.dry3g,
                dry4g: synth.dry4g,
                dry5g: synth.dry5g,
                dry6g: synth.dry6g,
                dry7g: synth.dry7g,
                wet0g: synth.wet0g,
                wet1g: synth.wet1g,
                wet2g: synth.wet2g,
                wet3g: synth.wet3g,
                wet4g: synth.wet4g,
                wet5g: synth.wet5g,
                wet6g: synth.wet6g,
                wet7g: synth.wet7g,
                f1_type: synth.f1_type,
                f1_cofo: synth.f1_cofo,
                f1_cofg: synth.f1_cofg,
                f1_resoo: synth.f1_resoo,
                f1_resog: synth.f1_resog,
                f2_type: synth.f2_type,
                f2_coflo: synth.f2_coflo,
                f2_coflg: synth.f2_coflg,
                f2_cofho: synth.f2_cofho,
                f2_cofhg: synth.f2_cofhg,
            }).unwrap(),
        ).unwrap();
    }

    let table_info = &csb.info;
    fs::write(
        output_directory_name.join("table_info.json"),
        serde_json::to_string(table_info).unwrap(),
    ).unwrap();

    for isaac in &csb.isaac {
        let file_path = output_directory_name.join(&isaac.ptname);
        create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(
            &file_path,
            serde_json::to_string(&isaac).unwrap(),
        ).unwrap();
    }

    println!("Extracted CSB file: \"{}\"", output_directory_name.to_string_lossy());
}