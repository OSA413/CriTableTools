use common_binary::error::CommonBinaryError;
use cri_table::table::{CriTable, UTF_SIGNATURE};

use crate::cpk::{etoc::CriCpkEToc, mode::CriCpkMode, toc::CriCpkToc};

pub struct CriCpkArchive
{
    pub update_date_time: u64,
    pub file_size: u64,
    pub content_offset: u64,
    pub content_size: u64,
    pub toc_offset: u64,
    pub toc_size: u64,
    pub toc_crc: u32,
    pub toc: Vec<CriCpkToc>,
    pub etoc_offset: u64,
    pub etoc_size: u64,
    pub etoc: Vec<CriCpkEToc>,
    // unused?
    pub itoc_offset: u64,
    pub itoc_size: u64,
    pub itoc_crc: u32,
    // unused?
    // there seems to be a lot of unused variables in terms of extraction and patching???
    pub gtoc_offset: u64,
    pub gtoc_size: u64,
    pub gtoc_crc: u32,
    pub enabled_packed_size: u64,
    pub enabled_data_size: u64,
    pub total_data_size: u64,
    pub tocs: u32,
    pub files: u32,
    pub groups: u32,
    pub attrs: u32,
    pub total_files: u32,
    pub directories: u32,
    pub updates: u32,
    pub version: u16,
    pub revision: u16,
    pub align: u16,
    pub sorted: u16,
    pub eid: u16,
    pub cpk_mode: u32,
    pub tvers: String,
    pub comment: String,
    pub codec: u32,
    pub dpk_itoc: u32,
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

    pub fn parse_toc_from_parent(source: &[u8], parent_table: &CriTable) -> Vec<CriCpkToc> {
        let toc_position = parent_table.get_field("TocOffset").unwrap().to_u64(0).unwrap();
        let toc_section_pointer = CriCpkArchive::get_section_pointer(source, toc_position as usize, *b"TOC ").unwrap();
        let table = CriTable::read(source, Some(toc_section_pointer), UTF_SIGNATURE).unwrap();
        
        let mut entries = Vec::with_capacity(table.row_count());
        for toc_row_index in 0..table.row_count() {
            let entry = CriCpkToc {
                dir_name: table.get_field("DirName").unwrap().to_u8(toc_row_index).unwrap(),
                file_name: table.get_field("FileName").unwrap().to_u8(toc_row_index).unwrap(),
                file_size: table.get_field("FileSize").unwrap().to_u8(toc_row_index).unwrap(),
                extract_size: table.get_field("ExtractSize").unwrap().to_u8(toc_row_index).unwrap(),
                file_offset: table.get_field("FileOffset").unwrap().to_u8(toc_row_index).unwrap(),
                id: table.get_field("ID").unwrap().to_u8(toc_row_index).unwrap(),
                user_string: table.get_field("UserString").unwrap().to_u8(toc_row_index).unwrap(),
            };
            entries.push(entry);
        }

        entries
    }

    pub fn parse_etoc_from_parent(source: &[u8], parent_table: &CriTable) -> Vec<CriCpkEToc> {
        let toc_position = parent_table.get_field("ETocOffset").unwrap().to_u64(0).unwrap();
        let toc_section_pointer = CriCpkArchive::get_section_pointer(source, toc_position as usize, *b"ETOC").unwrap();
        let table = CriTable::read(source, Some(toc_section_pointer), UTF_SIGNATURE).unwrap();
        
        let mut entries = Vec::with_capacity(table.row_count());
        for toc_row_index in 0..table.row_count() {
            let entry = CriCpkEToc {
                update_date_time: table.get_field("UpdateDateTime").unwrap().to_u64(toc_row_index).unwrap(),
                local_dir: table.get_field("LocalDir").unwrap().to_string(toc_row_index).unwrap(),
            };
            entries.push(entry);
        }

        entries
    }

    pub fn read(source: &[u8], pointer: usize) -> Result<CriCpkArchive, CommonBinaryError> {
        let cpk_section_pointer = CriCpkArchive::get_section_pointer(source, pointer, *b"CPK ")?;
        let table = CriTable::read(source, Some(cpk_section_pointer), UTF_SIGNATURE)?;

        assert_eq!(table.row_count(), 1, "It is expected for a CPK container to have exactly 1 row.");

        // From what I've seen, the tested games use the latest version and FileName mode
        let mode = match table.get_field("CpkMode") {
            Some(is_latest_version) => {
                CriCpkMode::from(is_latest_version.to_u32(0).unwrap())
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

        let toc = CriCpkArchive::parse_toc_from_parent(&source, &table);
        let etoc = CriCpkArchive::parse_etoc_from_parent(&source, &table);

        let update_date_time = table.get_field("UpdateDateTime").unwrap().to_u64(0).unwrap();
        let file_size = table.get_field("FileSize").unwrap().to_u64(0).unwrap();
        let content_offset = table.get_field("ContentOffset").unwrap().to_u64(0).unwrap();
        let content_size = table.get_field("ContentSize").unwrap().to_u64(0).unwrap();
        let toc_offset = table.get_field("TocOffset").unwrap().to_u64(0).unwrap();
        let toc_size = table.get_field("TocSize").unwrap().to_u64(0).unwrap();
        let toc_crc = table.get_field("TocCrc").unwrap().to_u32(0).unwrap();
        let etoc_offset = table.get_field("EtocOffset").unwrap().to_u64(0).unwrap();
        let etoc_size = table.get_field("EtocSize").unwrap().to_u64(0).unwrap();
        let itoc_offset = table.get_field("ItocOffset").unwrap().to_u64(0).unwrap();
        let itoc_size = table.get_field("ItocSize").unwrap().to_u64(0).unwrap();
        let itoc_crc = table.get_field("ItocCrc").unwrap().to_u32(0).unwrap();
        let gtoc_offset = table.get_field("GtocOffset").unwrap().to_u64(0).unwrap();
        let gtoc_size = table.get_field("GtocSize").unwrap().to_u64(0).unwrap();
        let gtoc_crc = table.get_field("GtocCrc").unwrap().to_u32(0).unwrap();
        let enabled_packed_size = table.get_field("EnabledPackedSize").unwrap().to_u64(0).unwrap();
        let enabled_data_size = table.get_field("EnabledDataSize").unwrap().to_u64(0).unwrap();
        let total_data_size = table.get_field("TotalDataSize").unwrap().to_u64(0).unwrap();
        let tocs = table.get_field("Tocs").unwrap().to_u32(0).unwrap();
        let files = table.get_field("Files").unwrap().to_u32(0).unwrap();
        let groups = table.get_field("Groups").unwrap().to_u32(0).unwrap();
        let attrs = table.get_field("Attrs").unwrap().to_u32(0).unwrap();
        let total_files = table.get_field("TotalFiles").unwrap().to_u32(0).unwrap();
        let directories = table.get_field("Directories").unwrap().to_u32(0).unwrap();
        let updates = table.get_field("Updates").unwrap().to_u32(0).unwrap();
        let version = table.get_field("Version").unwrap().to_u16(0).unwrap();
        let revision = table.get_field("Revision").unwrap().to_u16(0).unwrap();
        let align = table.get_field("Align").unwrap().to_u16(0).unwrap();
        let sorted = table.get_field("Sorted").unwrap().to_u16(0).unwrap();
        let eid = table.get_field("EID").unwrap().to_u16(0).unwrap();
        let cpk_mode = table.get_field("CpkMode").unwrap().to_u32(0).unwrap();
        let tvers = table.get_field("Tvers").unwrap().to_string(0).unwrap();
        let comment = table.get_field("Comment").unwrap().to_string(0).unwrap();
        let codec = table.get_field("Codec").unwrap().to_u32(0).unwrap();
        let dpk_itoc = table.get_field("DpkItoc").unwrap().to_u32(0).unwrap();

        Ok(CriCpkArchive {
            update_date_time,
            file_size,
            content_offset,
            content_size,
            toc_offset,
            toc_size,
            toc_crc,
            toc,
            etoc_offset,
            etoc_size,
            etoc,
            itoc_offset,
            itoc_size,
            itoc_crc,
            gtoc_offset,
            gtoc_size,
            gtoc_crc,
            enabled_packed_size,
            enabled_data_size,
            total_data_size,
            tocs,
            files,
            groups,
            attrs,
            total_files,
            directories,
            updates,
            version,
            revision,
            align,
            sorted,
            eid,
            cpk_mode,
            tvers,
            comment,
            codec,
            dpk_itoc,
        })
    }
}