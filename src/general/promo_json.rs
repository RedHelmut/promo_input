use crate::general::and_or::AndOrType;
use crate::general::promodatenativedef::PromoDateNative;
use crate::general::column_data::ColumnData;

#[derive(Deserialize, Clone)]
pub struct Promotion {
    pub start_date: PromoDateNative,
    pub end_date: PromoDateNative,
    pub promo_sections: Vec<PromoSection>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct PromoSection {
    #[serde(skip_serializing, skip_deserializing)]
    pub promo_parts_still_needed: Vec<usize>,
    #[serde(skip_serializing, skip_deserializing)]
    pub times_section_qualified: i64,
    pub part: Vec<Part>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Part {
    #[serde(rename = "type")] //, deserialize_with ="deserialize_type_ok_and_field"
    pub part_type: AndOrType,
    pub type_prod: Vec<TypeProd>,
    #[serde(skip_serializing, skip_deserializing)]
    pub qty_parts_still_needed: i64,
    #[serde(skip_serializing, skip_deserializing)]
    pub times_parts_qualified: i64,
    #[serde(skip_serializing, skip_deserializing)]
    pub type_prods_for_next_promo_needed: Vec<i64>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct TypeProd {
    pub qty_needed: f64,
    pub part_numbers: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub found_numbers: Vec<Vec<ColumnData>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub total_qty: i64,
    #[serde(skip_serializing, skip_deserializing)]
    pub times_qualified: i64,
}
