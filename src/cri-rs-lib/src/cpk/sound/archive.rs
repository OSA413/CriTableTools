use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use common_binary::error::CommonBinaryError;
use cri_table::table::{CriTable, UTF_SIGNATURE};

use crate::{aax::archive::CriAaxArchive, cpk::sound::{entry::CriCpkEntry, mode::CriCpkMode}};

#[derive(Debug)]
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
        let cpk_table = CriTable::read(source, Some(cpk_section_pointer), UTF_SIGNATURE)?;

        // From what I've seen, the tested games use the latest version and FileName mode
        let (mode, is_latest_version) = match cpk_table.get_field("CpkMode") {
            Some(is_latest_version) => {
                (CriCpkMode::from(is_latest_version.to_u32(0).unwrap()), true)
            }
            None => {
                eprintln!("Only latest version of CPK are supported, this file has no \"CpkMde\" field.");
                return Err(CommonBinaryError::SeeConsole());
            }
        };

        if mode != CriCpkMode::FileName {
            eprintln!("Unsupported CriCpkMode [{mode:?}]");
            return Err(CommonBinaryError::SeeConsole());
        }

        let toc_position = cpk_table.get_field("TocOffset").unwrap().to_u64(0)?;
        // let itocPosition = table.get_field("ItocOffset").unwrap().to_u64()?;
        // let etocPosition = table.get_field("EtocOffset").unwrap().to_u64()?;
        let content_position = cpk_table.get_field("ContentOffset").unwrap().to_u64(0)?;

        let toc_section_pointer = CriCpkArchive::get_section_pointer(source, toc_position as usize, *b"TOC ")?;
        let toc_table = CriTable::read(source, Some(toc_section_pointer), UTF_SIGNATURE)?;
        // let etoc_section_pointer = CriCpkArchive::get_section_pointer(source, etocPosition as usize, *b"ETOC")?;
        // let mut etocReader = CriTableHeader::read_table(source, Some(etoc_section_pointer), UTF_SIGNATURE)?;
        let comment = cpk_table.get_field("Comment").unwrap().to_string(0)?;

        let mut entries = Vec::with_capacity(toc_table.row_count());
        for toc_row_index in 0..toc_table.row_count() {
            let position = {
                let relative_position = toc_table.get_field("FileOffset").unwrap().to_u64(toc_row_index)? as usize;
                // Is this shift needed?
                if content_position < toc_position {
                    relative_position + content_position as usize
                } else {
                    relative_position + toc_position as usize
                }
            };
            
            let name = toc_table.get_field("FileName").unwrap().to_string(toc_row_index)?;
            let aax_table = CriTable::read(source, Some(position), UTF_SIGNATURE).unwrap();
            let aax_archive = CriAaxArchive::read(&aax_table, &name).unwrap();

            // etocReader.MoveToRow(tocReader.CurrentRow);
            // entry.UpdateDateTime = DateTimeFromCpkDateTime(etocReader.GetUInt64("UpdateDateTime"));

            let entry = CriCpkEntry {
                directory_name: toc_table.get_field("DirName").unwrap().to_string(toc_row_index)?,
                aax_archive,
                id: match is_latest_version {
                    true => toc_table.get_field("ID").unwrap().to_u32(toc_row_index)?,
                    false => toc_table.get_field("Info").unwrap().to_u32(toc_row_index)?,
                },
                comment: toc_table.get_field("UserString").unwrap().to_string(toc_row_index)?,
                update_date_time: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                ),
            };

            entries.push(entry);
        }

        Ok(CriCpkArchive { mode, comment, entries })
    }
}