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

pub fn get_shared(share_path: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let b = TagHandler::new(
        b"sst/si/t",
        |x: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::ColumnText(value, s, _) => {
                value.clone_from(&s.to_owned());
            }
            _ => {}
        },
    );
    //let afile_name = r##"F:\3M Promo Data\April-June 2019\may unfiltered\xl\sharedStrings.xml"##;
    let afile_name = share_path; //r##"F:\multiple table excel test\xl\sharedStrings.xml"##;
    let mut astd: String = String::new();
    let afile = File::open(afile_name)
        .expect("Err")
        .read_to_string(&mut astd);

    let mut sh: SharedDataShare = SharedDataShare::new();
    let mut r = xml_sheet_reader::create_parser::CreateParser::new(
        astd.as_str(),
        XmlReadingRange::Defined("A1:A*"),
        &mut sh,
        b"sst/si",
        Some(b"sst/si/t"),
        ParserFlags::ROWS_IN_SEQUENTIAL_ORDER_SKIP,
    );
    let mut cnt = 0;

    match r {
        Ok(ref mut v) => {
            v.add(b);

            for i in v {
                match i {
                    RowValidation::Valid(valid_row) => {
                        for k in valid_row {
                            ret.push(k);
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
    ret
}

pub fn get_shared_from_data(data: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();

    let b = TagHandler::new(
        b"sst/si/t",
        |_: &mut SharedDataShare, tag_type: InputTagType<String>| match tag_type {
            InputTagType::ColumnText(value, s, _) => {
                value.clone_from(&s.to_owned());
            }
            _ => {}
        },
    );
    //let afile_name = r##"F:\3M Promo Data\April-June 2019\may unfiltered\xl\sharedStrings.xml"##;

    let mut sh: SharedDataShare = SharedDataShare::new();
    let mut r = xml_sheet_reader::create_parser::CreateParser::new(
        data,
        XmlReadingRange::Defined("A1:A*"),
        &mut sh,
        b"sst/si",
        Some(b"sst/si/t"),
        ParserFlags::ROWS_IN_SEQUENTIAL_ORDER_SKIP,
    );
    let mut cnt = 0;

    match r {
        Ok(ref mut v) => {
            v.add(b);

            for i in v {
                match i {
                    RowValidation::Valid(valid_row) => {
                        for k in valid_row {
                            ret.push(k);
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
    ret
}
