use xml_sheet_reader::{StateInfo, DropRowRequest};
use crate::xlsx::share::Share;

#[derive(Clone)]
pub enum RequirementType<'a, T> {
    StringExactMatch(&'a str),
    DateRangeMatch(i64, i64),
    IsValueGreaterThan(T),
    IsValueLessThan(T),
    StartsWith(&'a str),
    IsOneOfItems(Vec<&'a str>),
    StringExactMatchIgnoreCase(&'a str),
    StartsWithIgnoreCase(&'a str),
    ContainsIgnoreCase(&'a str),
    Contains(&'a str),
}

pub fn test_requirements<D: ToString>(
    column: Option<String>,
    val: D,
    state: &StateInfo,
    x: &Share,
) -> bool {
    let value = val.to_string();
    if let Some(column_name) = column {
        if x.requirements.contains_key(&column_name) {
            match x.requirements[&column_name] {
                RequirementType::StringExactMatch(ref match_this) => {
                    if match_this != &value {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::StringExactMatchIgnoreCase(ref match_this) => {
                    if match_this.to_uppercase() != value.to_uppercase() {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::DateRangeMatch(ref start, ref end) => match value.parse::<i64>() {
                    Ok(parsed_date_xlsx) => {
                        if !(parsed_date_xlsx >= *start && parsed_date_xlsx <= *end) {
                            state.drop_row(DropRowRequest::Ignore);
                        }
                    }
                    Err(er) => {
                        let err = match state.get_column_name() {
                            Some(col) => format!("{} for column {}", er, col),
                            None => format!("{}", er),
                        };
                        state.drop_row(DropRowRequest::Error(err))
                    }
                },
                RequirementType::IsValueGreaterThan(ref g_than_val) => match value.parse::<f64>() {
                    Ok(parsed_date_xlsx) => {
                        if !(parsed_date_xlsx > *g_than_val) {
                            state.drop_row(DropRowRequest::Ignore);
                        }
                    }
                    Err(er) => {
                        let err = match state.get_column_name() {
                            Some(col) => format!("{} for column {}", er, col),
                            None => format!("{}", er),
                        };
                        state.drop_row(DropRowRequest::Error(err))
                    }
                },
                RequirementType::IsValueLessThan(ref g_than_val) => match value.parse::<f64>() {
                    Ok(parsed_date_xlsx) => {
                        if !(parsed_date_xlsx < *g_than_val) {
                            state.drop_row(DropRowRequest::Ignore);
                        }
                    }
                    Err(er) => {
                        let err = match state.get_column_name() {
                            Some(col) => format!("{} for column {}", er, col),
                            None => format!("{}", er),
                        };
                        state.drop_row(DropRowRequest::Error(err))
                    }
                },
                RequirementType::StartsWithIgnoreCase(ref starts_with) => {
                    let chk: String = starts_with.to_uppercase();
                    if !(value.to_uppercase().starts_with(chk.as_str())) {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::StartsWith(ref starts_with) => {
                    if !(value.starts_with(starts_with)) {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::IsOneOfItems(ref items) => {
                    if !items.contains(&value.as_str()) {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::Contains(ref data) => {
                    if !value.contains(data) {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
                RequirementType::ContainsIgnoreCase(ref data) => {
                    let mt = value.to_uppercase();
                    if !mt.contains(&data.to_uppercase()) {
                        state.drop_row(DropRowRequest::Ignore);
                    }
                }
            }
        }

        if state.is_dropping_row() {
            false
        } else {
            true
        }
    } else {
        false
    }
}
