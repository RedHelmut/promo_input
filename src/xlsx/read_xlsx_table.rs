use std::fmt;
//use xml_sheet_reader::TagHandler;
//use xml_sheet_reader::InputTagType;
//use xml_sheet_reader::RowValidation;
use super::table_info::TableInfo;
use std::fs::File;
use std::io::prelude::*;
use xml_sheet_reader::*;

#[derive(Clone)]
pub struct SharedDataShare {
    saved_dimensions: Option<String>,
    col_count: usize,
    table: TableInfo,
}
impl SharedDataShare {
    pub fn new() -> Self {
        Self {
            saved_dimensions: None,
            col_count: 0,
            table: TableInfo::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Table {
    pub table_columns: Vec<String>,
    pub table_info: TableInfo,
}
impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table Info: {}\r\nColumns: {:?}",
            self.table_info, self.table_columns
        )
    }
}
impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table Info: {}\r\nColumns: {:?}",
            self.table_info, self.table_columns
        )
    }
}

pub fn get_table(path: &str) -> Table {
    let b = TagHandler::new(
        b"table/tableColumns/tableColumn|name|",
        |_: &mut SharedDataShare, tag_type: InputTagType<String>| {
            match tag_type {
                InputTagType::ColumnStart(value, attribs, _) => {
                    if attribs.contains_key("name") {
                        let s = attribs["name"].clone();
                        //  cell_info.set_column_value( s.to_owned() );
                        value.clone_from(&s);
                    }
                }
                _ => {}
            }
        },
    );

    let z = TagHandler::new(
        b"table|name,displayName,ref|",
        |x: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::OtherTagStart(attribs, info) => {
                if attribs.contains_key("name") {
                    let s = attribs["name"].clone();
                    x.table.table_name = s.to_owned();
                }
                if attribs.contains_key("displayName") {
                    let s = attribs["displayName"].clone();
                    x.table.table_name = s.to_owned();
                }
                if attribs.contains_key("ref") {
                    let s = attribs["ref"].clone();
                    x.table.table_dimensions = s.to_owned();
                    info.set_dimensions(s.to_owned()).expect(&format!("Dimensions {}", s));
                }
            }
            _ => {}
        },
    );
    let c = TagHandler::new(
        b"table/tableColumns|count|",
        |x: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::ColumnStart(_, attribs, state) => {
                if !state.has_dimenstions() {
                    if attribs.contains_key("count") {
                        let s = attribs["count"].clone();
                        match s.to_owned().parse::<usize>() {
                            Ok(kk) => {
                                state.drop_row(DropRowRequest::Ignore);
                                x.saved_dimensions = Some(format!("A1:A{}", kk + 1));
                                state.set_dimensions("ddfds".to_owned());
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
            _ => {}
        },
    );
    let mut astd: String = String::new();
    //let afile = File::open(path).expect("Err").read_to_string(&mut astd);
    let mut sh: SharedDataShare = SharedDataShare::new();
    let mut r = xml_sheet_reader::create_parser::CreateParser::new(
        astd.as_str(),
        XmlReadingRange::Defined("A1:A*"),
        &mut sh,
        b"table/tableColumns/tableColumn",
        None,
        ParserFlags::ROWS_IN_SEQUENTIAL_ORDER_SKIP,
    );
    let mut cnt = 0;
    let mut column_table: Vec<String> = Vec::new();

    match r {
        Ok(ref mut v) => {
            v.add(b);
            v.add(c);
            v.add(z);
            for i in v {
                match i {
                    RowValidation::Valid(valid_row) => {
                        for k in valid_row {
                            column_table.push(k);
                            //print!("{}, ", k );
                        }
                    }
                    RowValidation::Invalid(in_valid_row) => {
                        println!("Error: {}", in_valid_row);
                    }
                    _ => {}
                }

                //println!("");
                cnt = cnt + 1;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    //println!("{}", cnt);
    let mut r_table = Table::default();
    r_table.table_columns = column_table;
    r_table.table_info = sh.table;
    r_table
}

pub fn get_table_from_string(data: &str) -> Table {
    let b = TagHandler::new(
        b"table/tableColumns/tableColumn|name|",
        |_: &mut SharedDataShare, tag_type: InputTagType<String>| {
            match tag_type {
                InputTagType::ColumnStart(value, attribs, state_info) => {
                    if attribs.contains_key("name") {
                        let s = attribs["name"].clone();
                        //  cell_info.set_column_value( s.to_owned() );
                        value.clone_from(&s);
                    }
                }
                _ => {}
            }
        },
    );

    let z = TagHandler::new(
        b"table|name,displayName,ref|",
        |x: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::OtherTagStart(attribs, info) => {
                if attribs.contains_key("name") {
                    let s = attribs["name"].clone();
                    x.table.table_name = s.to_owned();
                }
                if attribs.contains_key("displayName") {
                    let s = attribs["displayName"].clone();
                    x.table.table_name = s.to_owned();
                }
                if attribs.contains_key("ref") {
                    let s = attribs["ref"].clone();
                    x.table.table_dimensions = s.to_owned();
                    info.set_dimensions(s.to_owned());
                }
            }
            _ => {}
        },
    );
    let c = TagHandler::new(
        b"table/tableColumns|count|",
        |x: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::ColumnStart(_, attribs, state) => {
                if !state.has_dimenstions() {
                    if attribs.contains_key("count") {
                        let s = attribs["count"].clone();
                        match s.to_owned().parse::<usize>() {
                            Ok(kk) => {
                                state.drop_row(DropRowRequest::Ignore);
                                x.saved_dimensions = Some(format!("A1:A{}", kk + 1));
                                state.set_dimensions("ddfds".to_owned());
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
            _ => {}
        },
    );
    //let afile_name = r##"F:\3M Promo Data\April-June 2019\may unfiltered\xl\sharedStrings.xml"##;
    // let afile_name = r##"F:\3M Promo Data\April-June 2019\may unfiltered\xl\tables\table1.xml"##;
    // let mut astd:String = String::new();
    // let afile = File::open(path).expect("Err").read_to_string(&mut astd);
    let mut sh: SharedDataShare = SharedDataShare::new();
    let mut r = xml_sheet_reader::create_parser::CreateParser::new(
        data,
        XmlReadingRange::Defined("A1:A*"),
        &mut sh,
        b"table/tableColumns/tableColumn",
        None,
        ParserFlags::ROWS_IN_SEQUENTIAL_ORDER_SKIP,
    );
    let mut cnt = 0;
    let mut column_table: Vec<String> = Vec::new();

    match r {
        Ok(ref mut v) => {
            v.add(b);
            v.add(c);
            v.add(z);
            for i in v {
                match i {
                    RowValidation::Valid(valid_row) => {
                        for k in valid_row {
                            column_table.push(k);
                            //print!("{}, ", k );
                        }
                    }
                    RowValidation::Invalid(in_valid_row) => {
                        println!("Error: {}", in_valid_row);
                    }
                    _ => {}
                }

                //println!("");
                cnt = cnt + 1;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    //println!("{}", cnt);
    let mut r_table = Table::default();
    r_table.table_columns = column_table;
    r_table.table_info = sh.table;
    r_table
}
