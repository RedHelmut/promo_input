use xml_sheet_reader::{InputTagType, RowValidation, TagHandler,DropRowRequest};
use crate::general::column_data::ColumnData;
use crate::xlsx::share::Share;
use crate::general::insert_into_promo::{Pas, insert_into_promo};
use crate::xlsx::requirement_type::test_requirements;
use crate::general::xlsx_date_conversions::convert_from_xlsx_date_to_formatted_string;

pub fn create_tag_handlers<'a>() -> Vec<TagHandler<'a, Share<'a>, ColumnData>> {
    let a = TagHandler::new(
        b"worksheet/sheetData/row",
        |x: &mut Share, tag_type: InputTagType<ColumnData>| match tag_type {
            InputTagType::RowStart(attribs) => {}
            InputTagType::RowComplete(row) => match row {
                RowValidation::Valid(valid_row) => {
                    let customer_name = valid_row[x.customer_name_column_index].value.clone();
                    let qty = valid_row[x.qty_column_index].value.parse::<i64>().unwrap();
                    let pass = Pas {
                        item_name: valid_row,
                        part_number_index: x.part_number_column_index,
                        qty_index: x.qty_column_index,
                    };

                    match insert_into_promo(&mut x.complete_promo, customer_name, &x.promo, &pass) {
                        Ok(_) => {}
                        Err(e) => {
                            for i in e {
                                x.errors.push(format!("Error Parsing {} ", i));
                            }
                        }
                    }

                    x.total_qty = x.total_qty + qty;
                    x.total_sales = x.total_sales
                        + valid_row[x.sales_column_index]
                        .value
                        .parse::<f64>()
                        .unwrap();
                }
                _ => {}
            },
            InputTagType::HeaderComplete(r, fault_on_header) => {
                if r.contains_key("Qty") {
                    x.qty_column_index = r["Qty"];
                } else {
                    fault_on_header.cancel_operation("No 'Qty' header column".to_owned())
                }
                if r.contains_key("Sales") {
                    x.sales_column_index = r["Sales"];
                } else {
                    fault_on_header.cancel_operation("No 'Sales' header column".to_owned())
                }
                if r.contains_key("Related Customer Group") {
                    x.customer_name_column_index = r["Related Customer Group"];
                } else {
                    fault_on_header
                        .cancel_operation("No 'Related Customer Group' header column".to_owned())
                }
                if r.contains_key("Part Number") {
                    x.part_number_column_index = r["Part Number"];
                } else {
                    fault_on_header.cancel_operation("No 'Part Number' header column".to_owned())
                }
                if r.contains_key("Order Number") {
                    x.order_number_column_index = r["Order Number"];
                } else {
                    fault_on_header.cancel_operation("No 'Order Number' header column".to_owned())
                }
                if r.contains_key("Ship Date") {
                    x.ship_date_column_index = r["Ship Date"];
                }
                if r.contains_key("Part Number Description") {
                    x.part_number_desc_column_index = r["Part Number Description"];
                } else {
                    fault_on_header.cancel_operation("No 'Ship Date' header column".to_owned())
                }
            }
            _ => {}
        },
    );
    let b = TagHandler::new(
        b"worksheet/sheetData/row/c|t,r|",
        |x: &mut Share, tag_type: InputTagType<ColumnData>| match tag_type {
            InputTagType::ColumnStart(value, attribs, state) => {
                if attribs.contains_key("t") {
                    match attribs.get("t") {
                        Some(v) => {
                            x.last_col_type_attrib = v.to_owned();
                            value.add_attrib("t".to_owned(), v.to_owned());
                        }
                        None => {}
                    }
                } else if attribs.contains_key("r") {
                }
            }
            InputTagType::ColumnHeader(_, header_name, attribs, state) => {
                if attribs.contains_key("t") {
                    match attribs["t"].as_ref() {
                        "s" => {
                            let index = header_name.parse::<usize>();
                            match index {
                                Ok(valid_index) => {
                                    if valid_index < x.shared_strings.len() {
                                        *header_name = x.shared_strings[valid_index].clone();
                                    }
                                }
                                Err(er) => {
                                    let err = match state.get_column_name() {
                                        Some(col) => format!("{} for column {}", er, col),
                                        None => format!("{}", er),
                                    };
                                    state.drop_row(DropRowRequest::Error(err))
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            InputTagType::ColumnEnd(mut value, state) => {
                let column_name = state.get_column_name();
                if test_requirements(column_name.clone(), &mut value.value, &state, &x) {
                    let unwraped_column_name = column_name.unwrap();
                    match &x.last_col_type_attrib {
                        a if a == "s" => {
                            let index = value.value.parse::<usize>();
                            match index {
                                Ok(valid_index) => {
                                    if valid_index < x.shared_strings.len() {
                                        value.value = x.shared_strings[valid_index].clone();
                                    }
                                }
                                Err(er) => {
                                    let err = match state.get_column_name() {
                                        Some(col) => format!("{} for column {}", er, col),
                                        None => format!("{}", er),
                                    };
                                    state.drop_row(DropRowRequest::Error(err))
                                }
                            };
                        }
                        a if a == "str" => {
                            if unwraped_column_name == "Ship Date" {
                                if let Ok(valid_parse) = value.value.parse::<i64>() {
                                    value.value =
                                        convert_from_xlsx_date_to_formatted_string(valid_parse);
                                }
                            }
                        }
                        _ => {}
                    };
                }
            }

            _ => {}
        },
    );
    let c = TagHandler::new(
        b"worksheet/sheetData/row/c/v",
        |_: &mut Share, tag_type: InputTagType<ColumnData>| match tag_type {
            InputTagType::ColumnText(value, s, state) => {
                value.value = s.to_owned();
                match state.get_column_name() {
                    Some(name) => match name.as_ref() {
                        "Qty" => {}
                        "Sales" => {}
                        "Ship Date" => {}
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        },
    );

    let d = TagHandler::<Share, ColumnData>::new(
        b"worksheet/dimension|ref|",
        |_: &mut Share, tag_type: InputTagType<ColumnData>| match tag_type {
            InputTagType::OtherTagStart(attribs, info) => {
                if info.get_dimensions().is_none() {
                    if attribs.contains_key("ref") {
                        info.set_dimensions(attribs["ref"].clone());
                    }
                }
            }
            InputTagType::OtherTagEnd(_) => {}
            InputTagType::OtherTagText(_, _) => {}
            _ => {}
        },
    );
    vec![a, b, c, d]
}