use xml_sheet_reader::{RowValidation, XmlReadingRange, ParserFlags};
use std::error::Error;

use std::io::Read;
use crate::general::modify_promo::complete_promo;
use crate::general::column_data::ColumnData;
use crate::xlsx::requirement_type::RequirementType;
use crate::xlsx::share::Share;
use crate::xlsx::read_xlsx_share::get_shared_from_data;
use crate::xlsx::read_xlsx_table::get_table_from_string;

//use missing_report::display_missing_report;
use crate::general::promo_json::Promotion;
use std::collections::HashMap;
use std::fmt;
use crate::general::insert_into_promo::{insert_into_promo, Pas};
use crate::xlsx::init::create_tag_handlers;
use std::path::Path;
use std::fmt::{Display, Debug};
use serde::export::Formatter;
use chrono::{NaiveTime, NaiveDate, NaiveDateTime};
use std::str::FromStr;

pub struct PertInfo {
    pub data: HashMap<String, Promotion>,
    pub order_number_column_index: usize,
    pub customer_name_column_index: usize,
    pub part_number_column_index: usize,
    pub part_number_desc_column_index: usize,
    pub sales_column_index: usize,
    pub qty_column_index: usize,
    pub ship_date_column_index: usize,
}

pub trait FileInput {
    fn read(&mut self) -> Result<PertInfo, Box<dyn Error>>;
}



#[derive(Debug,Deserialize,Eq,Ord,PartialEq,PartialOrd,Clone,Serialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "Ship Date")]
    pub ship_date:String,
    #[serde( alias = "Related Customer Group", alias = "Customer Group Name", alias = "GroupName" )] //alias = "Customer Group Name",
    pub cust_group:String,
    // #[serde(alias = "'VENDABRV'[VEND]", alias = "Vend")]
    //pub vend:String,
    #[serde(alias = "Order*Ship", alias = "Order Number")]
    pub order_ship:String,
    #[serde(rename = "Part Number Description", alias = "Part Sku Desc")]
    pub part_num_desc:String,
    #[serde(rename = "Qty")]
    pub qty:String,
    #[serde(rename = "Sales")]
    pub sales:String,
    #[serde(alias = "Part Number")]
    pub part_number1:String,
    //#[serde(rename = "SHIP.ADDRESS1")]
    //pub ship_addr:String,
    //#[serde(rename = "SHIP.CITY")]
    //pub pgc:String,
    //#[serde(rename = "SHIP.STATE")]
    //pub ship_state:String,

}
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        write!(f, "Item {} - {}", self.part_number1 , self.qty)

    }
}
impl Record {
    pub fn make_vec_add_self(&self) -> Vec<Record> {
        let mut new_rec = Vec::new();
        new_rec.push(self.clone());
        new_rec
    }
}
pub struct CSVFile<'a> {
    csv_file_path: &'a str,
    json_promo_path: &'a str,
}
impl<'a> CSVFile<'a> {
    pub fn new(csv_file_path: &'a str, json_promo_path: &'a str) -> Self {
        Self {
            csv_file_path,
            json_promo_path
        }
    }
}
impl<'a> FileInput for CSVFile<'a> {
    fn read(&mut self) -> Result<PertInfo, Box<dyn Error>> {

        //  let mut data: HashMap<String, Promotion> = HashMap::new();
        let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(self.csv_file_path)?;
        let mut sh: Share = Share::new(self.json_promo_path)?;

        //sh.order_number_column_index = 2;
        sh.customer_name_column_index = 1;
        sh.part_number_column_index = 3;
        sh.part_number_desc_column_index = 0;
        sh.sales_column_index = 5;
        sh.qty_column_index = 4;
        sh.ship_date_column_index = 2;
        sh.order_number_column_index = 6;

        for r in reader.deserialize() {
            let mut rec: Record = r?;
            //, rec.ship_addr, rec.ship_date,rec.ship_state, rec.vend, rec.pgc
     //      println!("{}", rec.ship_date.clone());
//            println!("PND {}, CN {}, SD {}, $ {}", rec.part_num_desc.clone(), rec.cust_group.clone(), rec.ship_date.clone(), rec.part_number1.clone());

            if rec.ship_date.is_empty() || rec.cust_group.is_empty() {
                continue;
            }

            let nt = NaiveDateTime::parse_from_str(&rec.ship_date, "%Y-%m-%d %H:%M:%S");
            if let Ok(e) = nt {
                rec.ship_date = e.format("%Y-%m-%d").to_string();
            } else {
                unimplemented!();
                //println!("Fail Parsing {}", rec.ship_date);
            }

            if rec.sales.contains('$') {
                rec.sales = rec.sales.replace('$', "");
            }
            let mut altered_row = vec![rec.part_num_desc, rec.cust_group.clone(), rec.ship_date, rec.part_number1, rec.qty, rec.sales, rec.order_ship]
                .into_iter()
                .map(|x| ColumnData::new(x, HashMap::new()))
                .collect::<Vec<ColumnData>>();


            let pass = Pas {
                item_name: &mut altered_row,
                part_number_index: 3,
                qty_index: 4,
            };
            insert_into_promo(&mut sh.complete_promo, rec.cust_group.clone(), &sh.promo, &pass);
        }

        complete_promo(&mut sh.complete_promo);

        let pert = PertInfo{
            data: sh.complete_promo,
            order_number_column_index: 6,
            customer_name_column_index : 1,
            part_number_column_index : 3,
            part_number_desc_column_index: 0,
            sales_column_index : 5,
            qty_column_index : 4,
            ship_date_column_index : 2,
        };

//        let d = sh.complete_promo[&0].promo_sections[0].part[0]..clone();
      //  println!("{}",d);
       // println!("Order Num: {}, Ship Date: {}", sh.complete_promo[pert.order_number_column_index],sh.complete_promo[0][pert.ship_date_column_index] );
        Ok(pert)
    }
}

pub struct XLSXFile<'a> {
    xlsx_file: &'a str,
    json_promo_file: &'a str,
}
impl<'a> XLSXFile<'a> {
    pub fn new(xlsx_file: &'a str, json_promo_file: &'a str) -> Self {
        Self {
            xlsx_file,
            json_promo_file
        }
    }
}
impl<'a> FileInput for XLSXFile<'a> {
    fn read(&mut self) -> Result<PertInfo, Box<dyn Error>> {
        let customer: Option<RequirementType<f64>> = None;
        //   let mut json_promo_file: &str = r##"F:\3M Promo Data\May1-July31 2020\promo_May1_2020-July31_2020.json"##;
        //   let mut xlsx_file: &str = r##"F:\3M Promo Data\May1-July31 2020\ViewExport Customer Detail.xlsx"##;
        let mut sh: Share = Share::new(self.json_promo_file)?;


        let rdr = std::fs::read(self.xlsx_file)?;
        let reader = std::io::Cursor::new(rdr);
        let mut zip = zip::ZipArchive::new(reader)?;
        let mut table_string: String = String::new();
        {
            let mut table1 = zip.by_name("xl/tables/table1.xml")?;
            table1.read_to_string(&mut table_string)?;
        }
        let mut share_string: String = String::new();
        {
            let mut share_file = zip.by_name("xl/sharedStrings.xml")?;
            share_file.read_to_string(&mut share_string)?;
        }
        let mut sheet_string: String = String::new();
        {
            let mut sheet1 = zip.by_name("xl/worksheets/sheet1.xml")?;
            sheet1.read_to_string(&mut sheet_string)?;
        }

        sh.shared_strings = get_shared_from_data(share_string.as_ref());
        sh.table_data = get_table_from_string(table_string.as_ref());
        if let Some(cust) = customer {
            sh.requirements
                .entry("Related Customer Group".to_owned())
                .or_insert(cust);
        }

        let mut r = xml_sheet_reader::create_parser::CreateParser::<Share, ColumnData>::new(
            sheet_string.as_ref(),
            XmlReadingRange::Defined(sh.table_data.table_info.table_dimensions.clone().as_str()),
            &mut sh,
            b"worksheet/sheetData/row|r|",
            Some(b"worksheet/sheetData/row/c|r|"),
            ParserFlags::RETURN_HEADER
                | ParserFlags::HAS_HEADER
                | ParserFlags::MODIFY_HEADER
                | ParserFlags::ROWS_IN_SEQUENTIAL_ORDER_SKIP,
        );

        match r {
            Ok(ref mut v) => {
                v.add_many(create_tag_handlers());

                let (invalid_rows, valid_rows): (Vec<_>, Vec<_>) = v.partition(|x| x.is_invalid());
                if invalid_rows.len() > 0 {
                    for i in invalid_rows
                        .into_iter()
                        .filter_map(RowValidation::get_invalid)
                        .collect::<Vec<_>>()
                    {
                        println!("{}", i.message);
                    }
                } else {
                    for i in valid_rows {
                        match i {
                            RowValidation::Valid(_) => {}
                            RowValidation::Invalid(_) => {}
                            RowValidation::Header(_) => {}
                            RowValidation::CriticalFail(_) => {}
                        }
                    }
                }
            }
            Err(_) => {}
        }

        complete_promo(&mut sh.complete_promo);
        let pert = PertInfo{
            data: sh.complete_promo,
            order_number_column_index: sh.order_number_column_index,
            customer_name_column_index: sh.customer_name_column_index,
            part_number_column_index: sh.part_number_column_index,
            part_number_desc_column_index: sh.part_number_desc_column_index,
            sales_column_index: sh.sales_column_index,
            qty_column_index: sh.qty_column_index,
            ship_date_column_index: sh.ship_date_column_index,
        };
        Ok(pert)
    }
}

pub struct LoadError {
    file_type: String
}
impl LoadError {
    pub fn new(file_type:String) -> Self {
        Self {
            file_type
        }
    }
}
impl Error for LoadError  {

}
impl  Display for LoadError  {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File canno't be of type: {}", self.file_type)
    }
}
impl Debug for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File canno't be of type: {}, only looks at extention of file.", self.file_type)
    }
}
pub fn load_promo<'b>( file_path: &'b str, promo_path: &'b str) -> Result<PertInfo,Box<dyn Error>> {
    let path = Path::new( file_path );
    match path.extension() {
        Some( ext ) if ext == "csv" => {
            let mut csv = CSVFile::new(file_path, promo_path);
            csv.read()
        },
        Some( ext ) if ext == "xlsx" => {
            let mut xlsx = XLSXFile::new(file_path, promo_path);
            xlsx.read()
        },
        Some(tpt) => {
            let message = tpt.to_string_lossy();
            let kr = message.as_bytes();
            let msg = String::from_utf8(kr.to_vec()).unwrap_or("Unknown".to_owned());
            Err(Box::new(LoadError::new(msg)))
        },
        None => {
            Err(Box::new(LoadError::new(".csv or .xlsx file name required!".to_owned())))
        }
    }

}