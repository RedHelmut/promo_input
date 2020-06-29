use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
pub struct ColumnData {
    pub value: String,
    pub saved_attribs: HashMap<String, String>,
    pub type_of_data: String,
}
impl ColumnData {
    pub fn new(val: String, val_type: HashMap<String, String>) -> Self {
        Self {
            value: val,
            saved_attribs: val_type,
            type_of_data: String::new(),
        }
    }
    pub fn add_attrib(&mut self, attrib: String, value: String) {
        self.saved_attribs.entry(attrib).or_insert(value);
    }
}

impl std::fmt::Display for ColumnData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}({:?})", self.value, self.saved_attribs)
        write!(f, "{}", self.value)
    }
}
