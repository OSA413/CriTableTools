use common_binary::error::CommonBinaryError;
use cri_table::table::CriTable;

pub trait FromCriTable {
    fn from_table(table: &CriTable) -> Result<Self, CommonBinaryError> where Self: Sized;
}

pub trait ToCriTable {
    fn to_table(&self) -> Result<CriTable, CommonBinaryError>;

    fn to_bytes(&self) -> Result<Vec<u8>, CommonBinaryError> {
        self.to_table()?.write()
    }
}