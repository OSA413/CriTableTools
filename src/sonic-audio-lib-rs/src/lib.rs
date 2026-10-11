pub mod archives {
    pub mod cri {
        pub mod aax {
            pub mod archive_mode;
            pub mod archive;
            pub mod entry_flag;
            pub mod entry;
        }
        pub mod cpk {
            pub mod archive;
            pub mod mode;
            pub mod entry;
        }
        pub mod mw {
            pub mod table {
                pub mod common;
                pub mod field_type;
                pub mod field_flags;
                pub mod field_raw_flags;
                pub mod field_value;
                pub mod field;
                pub mod header_encoding_type;
                pub mod table_reading_result;
                pub mod table;
            }
        }
    }
}
