/*
Please not that this is only a subset of all possible CriTables

This hexpat is based off of Skyth's SonicAudioTools' Lib
That's why it's also licensed under the MIT license

This hexpat is MIT licensed:
    Copyright (c) 2019 Skyth
    Copyright (c) 2026 Oleg "OSA413" Sokolov
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

#pragma endian big

import std.core;
import std.io;

enum CriFieldRawFlags: u8 {
    Name = 16,
    DefaultValue = 32,
    RowStorage = 64,

    Byte = 0,
    SByte = 1,
    UInt16 = 2,
    Int16 = 3,
    UInt32 = 4,
    Int32 = 5,
    UInt64 = 6,
    Int64 = 7,
    Single = 8,
    Double = 9,
    String = 10,
    Data = 11,
    Guid = 12,
};

struct CriTableSignature {
    char magic[0x4];
    u32 length;
};

struct CriTableInfo {
    u8 unknown_byte;
    u8 encoding_type;
    u16 row_position;
    u32 string_pool_position;
    u32 data_pool_position;
    // This is the relative position from string pool start
    u32 table_name_position;
    u16 field_count;
    u16 row_length;
    u32 row_count;
    
    // 8 is the size of the signature
    char table_name[] @ table_name_position + string_pool_position + 8;
};

struct CriTableRowDefaultValue {
    u8 field_raw_flags = parent.field_raw_flags;

    if (field_raw_flags & CriFieldRawFlags::DefaultValue == CriFieldRawFlags::DefaultValue) {
        if (field_raw_flags & CriFieldRawFlags::Guid == CriFieldRawFlags::Guid) {
            u128 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Data == CriFieldRawFlags::Data) {
            u32 default_value_pad;
            u32 default_value_pad2;
        } else if (field_raw_flags & CriFieldRawFlags::String == CriFieldRawFlags::String) {
            u32 string_pointer;
            char default_value[] @ string_pointer
                + parent.parent.table_info.string_pool_position
                + 0x08;
        } else if (field_raw_flags & CriFieldRawFlags::Double == CriFieldRawFlags::Double) {
            double default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Single == CriFieldRawFlags::Single) {
            float default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int64 == CriFieldRawFlags::Int64) {
            s64 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt64 == CriFieldRawFlags::UInt64) {
            u64 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int32 == CriFieldRawFlags::Int32) {
            s32 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt32 == CriFieldRawFlags::UInt32) {
            u32 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int16 == CriFieldRawFlags::Int16) {
            s16 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt16 == CriFieldRawFlags::UInt16) {
            u16 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::SByte == CriFieldRawFlags::SByte) {
            s8 default_value;
        } else if (field_raw_flags & CriFieldRawFlags::Byte == CriFieldRawFlags::Byte) {
            u8 default_value;
        } else {
            u8 panic = 1/0;
        }
    }
};

struct CriTableRowStorage {
    u8 field_raw_flags = parent.parent.table_fields_info[std::core::array_index()].field_raw_flags;

    if (field_raw_flags & CriFieldRawFlags::RowStorage == CriFieldRawFlags::RowStorage) {
        if (field_raw_flags & CriFieldRawFlags::Guid == CriFieldRawFlags::Guid) {
            u128 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Data == CriFieldRawFlags::Data) {
            u32 row_storage_data_pointer;
            u32 row_storage_data_length;
            u32 absolute_data_pointer = parent.parent.table_info.data_pool_position
                + row_storage_data_pointer
                + 0x8;
            u8 row_storage_value[row_storage_data_length] @ absolute_data_pointer;
        } else if (field_raw_flags & CriFieldRawFlags::String == CriFieldRawFlags::String) {
            u32 string_pointer;
            char row_storage_value[] @ 0x08
                + string_pointer
                + parent.parent.table_info.string_pool_position;
        } else if (field_raw_flags & CriFieldRawFlags::Double == CriFieldRawFlags::Double) {
            double row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Single == CriFieldRawFlags::Single) {
            float row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int64 == CriFieldRawFlags::Int64) {
            s64 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt64 == CriFieldRawFlags::UInt64) {
            u64 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int32 == CriFieldRawFlags::Int32) {
            s32 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt32 == CriFieldRawFlags::UInt32) {
            u32 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Int16 == CriFieldRawFlags::Int16) {
           s16 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::UInt16 == CriFieldRawFlags::UInt16) {
           u16 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::SByte == CriFieldRawFlags::SByte) {
           s8 row_storage_value;
        } else if (field_raw_flags & CriFieldRawFlags::Byte == CriFieldRawFlags::Byte) {
           u8 row_storage_value;
        } else {
           u8 panic = 1/0;
        }
    }
};

struct CriTableRowWrapper {
    CriTableRowStorage storage[parent.table_info.field_count];
};

struct CriTableFieldsInfo {
    u8 field_raw_flags;
    u32 field_name_pointer;
    char field_name[] @ field_name_pointer
        + parent.table_info.string_pool_position
        + 0x8;

    CriTableRowDefaultValue field_default_value;
};

struct CriTable {
    CriTableSignature signature;
    CriTableInfo table_info;
    CriTableFieldsInfo table_fields_info[table_info.field_count];
    CriTableRowWrapper rows[table_info.row_count];
};

CriTable cri_table @ 0x00;
