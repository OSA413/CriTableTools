use bitflags::bitflags;

use crate::archives::cri::mw::table::{field_type::CriFieldType};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CriFieldFlag: u8 {
        const Name = 16;
        const DefaultValue = 32;
        const RowStorage = 64;
        
        const Byte = 0;
        const SByte = 1;
        const UInt16 = 2;
        const Int16 = 3;
        const UInt32 = 4;
        const Int32 = 5;
        const UInt64 = 6;
        const Int64 = 7;
        const Single = 8;
        const Double = 9;
        const String = 10;
        const Data = 11;
        const Guid = 12;
    }
}

impl CriFieldFlag {
    pub fn extract_type(&self) -> CriFieldType {
        return if self.contains(CriFieldFlag::Guid) {CriFieldType::Guid}
        else if self.contains(CriFieldFlag::Data) {CriFieldType::Data}
        else if self.contains(CriFieldFlag::String) {CriFieldType::String}
        else if self.contains(CriFieldFlag::Double) {CriFieldType::Double}
        else if self.contains(CriFieldFlag::Single) {CriFieldType::Single}
        else if self.contains(CriFieldFlag::Int64) {CriFieldType::Int64}
        else if self.contains(CriFieldFlag::UInt64) {CriFieldType::UInt64}
        else if self.contains(CriFieldFlag::Int32) {CriFieldType::Int32}
        else if self.contains(CriFieldFlag::UInt32) {CriFieldType::UInt32}
        else if self.contains(CriFieldFlag::Int16) {CriFieldType::Int16}
        else if self.contains(CriFieldFlag::UInt16) {CriFieldType::UInt16}
        else if self.contains(CriFieldFlag::SByte) {CriFieldType::SByte}
        else if self.contains(CriFieldFlag::Byte) {CriFieldType::Byte}
        else { panic!("At the dance floor, {self:?}") };
    }
}
