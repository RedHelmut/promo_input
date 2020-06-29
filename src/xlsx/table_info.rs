use std::fmt;

#[derive(Default, Clone)]
pub struct TableInfo {
    pub table_dimensions: String,
    pub table_name: String,
}
impl fmt::Display for TableInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table Name: {}\r\nDimensions: {}",
            self.table_name, self.table_dimensions
        )
    }
}
impl fmt::Debug for TableInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table Name: {}\r\nDimensions: {}",
            self.table_name, self.table_dimensions
        )
    }
}
