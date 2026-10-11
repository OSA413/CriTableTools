use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use common_binary::error::CommonBinaryError;

use crate::archives::cri::{cpk::{entry::CriCpkEntry, mode::CriCpkMode}, mw::table::table::CriTable};

pub struct CriCpkArchive
{
    pub mode: CriCpkMode,
    pub comment: String, 
    pub entries: Vec<CriCpkEntry>,
}

impl CriCpkArchive {
    fn get_section_pointer(source: &[u8], position: usize, expected_signature: [u8; 4]) -> Result<usize, CommonBinaryError> {
        if !source[position..position + 4].eq(&expected_signature) {
            eprintln!("Invalid signature");
            return Err(CommonBinaryError::SeeConsole());
        }

        // In the original C# project there's a read of signature, flag, tableLength, and the "unknown", each u32;
        // Those values don't seem to be used, but the signature check looks very helpfull (anyway, that's what the original does)

        Ok(position + 0x10)
    }

    pub fn read(source: &[u8], pointer: usize) -> Result<CriCpkArchive, CommonBinaryError> {
        let cpk_section_pointer = CriCpkArchive::get_section_pointer(source, pointer, *b"CPK ")?;
        let reader = CriTable::read(source, Some(cpk_section_pointer), *b"@UTF")?;

        let (mode, is_latest_version) = match reader.table.get_field("CpkMode") {
            Some(is_latest_version) => {
                (CriCpkMode::from(is_latest_version.to_u32(0).unwrap()), true)
            }
            None => {
                let toc_enabled = reader.table.get_field("TocOffset").is_some_and(|x| x.to_u64(0).unwrap() > 0);
                let itoc_enabled = reader.table.get_field("ItocOffset").is_some_and(|x| x.to_u64(0).unwrap() > 0);

                if toc_enabled && !itoc_enabled {
                    (CriCpkMode::FileName, false)
                } else if !toc_enabled && itoc_enabled {
                    (CriCpkMode::Id, false)
                } else if toc_enabled && itoc_enabled {
                    (CriCpkMode::FileNameAndId, false)
                } else {
                    (CriCpkMode::None, false)
                }
            }
        };

        // No need to waste time, stop right there.
        if mode == CriCpkMode::None {
            return Ok(CriCpkArchive {
                mode,
                comment: "".to_string(),
                entries: vec![],
            })
        }

        // Why is this u64?
        let toc_position = reader.table.get_field("TocOffset").unwrap().to_u64(0)?;
        // let itocPosition = reader.table.get_field("ItocOffset").unwrap().to_u64()?;
        // let etocPosition = reader.table.get_field("EtocOffset").unwrap().to_u64()?;
        let content_position = reader.table.get_field("ContentOffset").unwrap().to_u64(0)?;

        // let align = reader.table.get_field("Align").unwrap().to_u16()?;

        let mut entries = vec![];

        if mode == CriCpkMode::FileName
        {
            let toc_section_pointer = CriCpkArchive::get_section_pointer(source, toc_position as usize, *b"TOC ")?;
            let toc_reader = CriTable::read(source, Some(toc_section_pointer), *b"@UTF")?;
            // let etoc_section_pointer = CriCpkArchive::get_section_pointer(source, etocPosition as usize, *b"ETOC")?;
            // let mut etocReader = CriTableHeader::read_table(source, Some(etoc_section_pointer), *b"@UTF")?;

            for toc_row_index in 0..toc_reader.row_count() {
                let length = toc_reader.table.get_field("FileSize").unwrap().to_u32(toc_row_index)? as usize;
                let position = {
                    let relative_position = toc_reader.table.get_field("FileOffset").unwrap().to_u64(toc_row_index)? as usize;
                    if content_position < toc_position {
                        relative_position + content_position as usize
                    } else {
                        relative_position + toc_position as usize
                    }
                };
                let uncompressed_length = toc_reader.table.get_field("ExtractSize").unwrap().to_u32(toc_row_index)? as usize;
                let is_compressed = length != uncompressed_length;

                // etocReader.MoveToRow(tocReader.CurrentRow);
                //entry.UpdateDateTime = DateTimeFromCpkDateTime(etocReader.GetUInt64("UpdateDateTime"));

                let entry = CriCpkEntry {
                    directory_name: toc_reader.table.get_field("DirName").unwrap().to_string(toc_row_index)?,
                    name: toc_reader.table.get_field("FileName").unwrap().to_string(toc_row_index)?,
                    value: source[position..position + length].to_vec(),
                    id: match is_latest_version {
                        true => toc_reader.table.get_field("ID").unwrap().to_u32(toc_row_index)?,
                        false => toc_reader.table.get_field("Info").unwrap().to_u32(toc_row_index)?,
                    },
                    comment: toc_reader.table.get_field("UserString").unwrap().to_string(toc_row_index)?,
                    uncompressed_length: toc_reader.table.get_field("ExtractSize").unwrap().to_u32(toc_row_index)?,
                    update_date_time: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                    ),
                    is_compressed,
                };

                entries.push(entry);
            }
        } else {
            eprintln!("Unimplemented CPK mode ({mode:?})");
            return Err(CommonBinaryError::SeeConsole());
        }

        let comment = reader.table.get_field("Comment").unwrap().to_string(0)?;

        Ok(CriCpkArchive { mode, comment, entries })
    }

    pub fn get_by_path(&self, path: &str) -> Option<&CriCpkEntry> {
        let corrected_path = path.replace("\\", "/");

        return self.entries.iter().find(|entry| {
            let search = match entry.directory_name.is_empty() {
                true => entry.name.clone(),
                false => format!("{}/{}", entry.directory_name.replace("\\", "/"), entry.name),
            };

            return search == corrected_path;
        });
    }
}