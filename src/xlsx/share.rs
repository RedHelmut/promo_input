use std::collections::HashMap;
use crate::general::promo_json::Promotion;
use crate::xlsx::requirement_type::RequirementType;
use crate::xlsx::read_xlsx_table::Table;
use crate::general::xlsx_date_conversions::convert_from_promo_date_to_excel_format;

pub struct Share<'a> {
    pub last_col_type_attrib: String,
    pub shared_strings: Vec<String>,
    pub dimensions: Option<String>,
    pub total_qty: i64,
    pub total_sales: f64,
    pub table_data: Table,
    pub requirements: HashMap<String, RequirementType<'a, f64>>,
    pub promo: Promotion,
    pub complete_promo: HashMap<String, Promotion>,
    pub current_customer_name: String,
    pub qty_column_index: usize,
    pub sales_column_index: usize,
    pub customer_name_column_index: usize,
    pub part_number_column_index: usize,
    pub order_number_column_index: usize,
    pub ship_date_column_index: usize,
    pub part_number_desc_column_index: usize,
    pub errors: Vec<String>,
}
impl<'a> Share<'a> {
    pub fn new(json_file: &str) -> Result<Self, String> {
        //r##"F:\3M Promo Data\April-June 2019\promo_april2019-june2019.json"##
        let open_file = std::fs::File::open(json_file.clone())
            .map_err(|x:std::io::Error| format!("Failure opening {} : {}", json_file, x.to_string()))?;
        let parsed_json =
            serde_json::from_reader(open_file).map_err(|x| format!("Error parsing Json file: {}", x.to_string()))?;
        let promo: Promotion = parsed_json;
        let start_date = promo.start_date.clone().to_string();
        let end_date = promo.end_date.clone().to_string();

        let mut s = Self {
            last_col_type_attrib: String::default(),
            shared_strings: Vec::new(),
            dimensions: None,
            total_qty: 0,
            total_sales: 0.0,
            table_data: Table::default(),
            requirements: HashMap::new(),
            promo: promo,
            complete_promo: HashMap::new(),
            current_customer_name: String::new(),
            qty_column_index: 0,
            sales_column_index: 0,
            customer_name_column_index: 0,
            part_number_column_index: 0,
            order_number_column_index: 0,
            ship_date_column_index: 0,
            part_number_desc_column_index: 0,
            errors: Vec::new(),
        };
        s.requirements
            .entry("Ship Date".to_owned())
            .or_insert(RequirementType::DateRangeMatch(
                convert_from_promo_date_to_excel_format(start_date).unwrap(),
                convert_from_promo_date_to_excel_format(end_date).unwrap(),
            ));
        //  s.requirements.entry("VEND".to_owned()).or_insert(RequirementType::StringExactMatch("3M".to_owned()));
        Ok(s)
    }
}
