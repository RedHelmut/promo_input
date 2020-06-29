use chrono::NaiveDate;

pub fn convert_from_date_to_excel_format_string(
    date: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use chrono::NaiveDate;

    let ch: NaiveDate = NaiveDate::parse_from_str(date, "%m/%d/%Y")?;
    let o_date = NaiveDate::from_ymd(1899, 12, 30);
    let d: chrono::Duration = ch - o_date;
    Ok(d.num_days().to_string())
}
pub fn convert_from_date_string_to_excel_format_string(
    date: String,
) -> Result<String, Box<dyn std::error::Error>> {
    use chrono::NaiveDate;

    let ch: NaiveDate = NaiveDate::parse_from_str(date.as_ref(), "%m/%d/%Y")?;
    let o_date = NaiveDate::from_ymd(1899, 12, 30);
    let d: chrono::Duration = ch - o_date;
    Ok(d.num_days().to_string())
}
pub fn convert_from_date_to_excel_format(date: &str) -> Result<i64, Box<dyn std::error::Error>> {
    use chrono::NaiveDate;

    let ch: NaiveDate = NaiveDate::parse_from_str(date, "%m/%d/%Y")?;
    let o_date = NaiveDate::from_ymd(1899, 12, 30);
    let d: chrono::Duration = ch - o_date;
    Ok(d.num_days())
}
pub fn convert_from_promo_date_to_excel_format(
    date: String,
) -> Result<i64, Box<dyn std::error::Error>> {
    use chrono::NaiveDate;

    let ch: NaiveDate = NaiveDate::parse_from_str(date.as_ref(), "%m-%d-%Y")?;
    let o_date = NaiveDate::from_ymd(1899, 12, 30);
    let d: chrono::Duration = ch - o_date;
    Ok(d.num_days())
}
pub fn convert_from_xlsx_date_to_formatted_string(xlsx_date: i64) -> String {
    let beginning = NaiveDate::from_ymd(1899, 12, 30);
    let dayz = chrono::Duration::days(xlsx_date);
    let new_date = beginning + dayz;
    new_date.format("%m/%d/%Y").to_string()
}
