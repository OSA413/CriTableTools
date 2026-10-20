use bitflags::bitflags;

use crate::{field_flags::CriFieldFlags, field_type::CriFieldType};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct CriFieldRawFlags: u8 {
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

impl CriFieldRawFlags {
    pub fn extract_type(&self) -> CriFieldType {
        if self.contains(CriFieldRawFlags::Guid) {CriFieldType::Guid}
        else if self.contains(CriFieldRawFlags::Data)
            && self.contains(CriFieldRawFlags::RowStorage) {CriFieldType::ProbablyTable}
        else if self.contains(CriFieldRawFlags::Data) {CriFieldType::Data}
        else if self.contains(CriFieldRawFlags::String) {CriFieldType::String}
        else if self.contains(CriFieldRawFlags::Double) {CriFieldType::Double}
        else if self.contains(CriFieldRawFlags::Single) {CriFieldType::Single}
        else if self.contains(CriFieldRawFlags::Int64) {CriFieldType::Int64}
        else if self.contains(CriFieldRawFlags::UInt64) {CriFieldType::UInt64}
        else if self.contains(CriFieldRawFlags::Int32) {CriFieldType::Int32}
        else if self.contains(CriFieldRawFlags::UInt32) {CriFieldType::UInt32}
        else if self.contains(CriFieldRawFlags::Int16) {CriFieldType::Int16}
        else if self.contains(CriFieldRawFlags::UInt16) {CriFieldType::UInt16}
        else if self.contains(CriFieldRawFlags::SByte) {CriFieldType::SByte}
        else if self.contains(CriFieldRawFlags::Byte) {CriFieldType::Byte}
        else { panic!("At the dance floor, {self:?}") }
    }

    pub fn extract_flags(&self) -> CriFieldFlags {
        CriFieldFlags {
            name: self.contains(CriFieldRawFlags::Name),
            default_value: self.contains(CriFieldRawFlags::DefaultValue),
            row_storage: self.contains(CriFieldRawFlags::RowStorage),
        }
    }

    pub fn from_flags_and_type(flags: &CriFieldFlags, ttype: &CriFieldType) -> CriFieldRawFlags {
        let value = match flags.default_value { true => 1, false => 0 } * CriFieldRawFlags::DefaultValue.bits()
            + match flags.row_storage { true => 1, false => 0 } * CriFieldRawFlags::RowStorage.bits()
            + match flags.name { true => 1, false => 0 } * CriFieldRawFlags::Name.bits()
            + ttype.get_bits();
        CriFieldRawFlags::from_bits(value).unwrap()
    }
}
