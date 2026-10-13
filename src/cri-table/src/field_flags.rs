#[derive(Debug, Clone, Copy)]
pub struct CriFieldFlags {
    pub name: bool,
    pub default_value: bool,
    pub row_storage: bool,
}
