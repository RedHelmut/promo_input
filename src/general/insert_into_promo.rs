use std::collections::HashMap;
use crate::general::column_data::ColumnData;
use crate::general::promo_json;


pub struct Pas<'a> {
    pub item_name: &'a mut Vec<ColumnData>,
    pub part_number_index: usize,
    pub qty_index: usize,
    //   date_purchased: NaiveTime<Utc>,
}

///////////////

fn insert_into_promo_section(
    promo_seg: &mut promo_json::PromoSection,
    p: &Pas,
) -> Result<(), std::num::ParseIntError> {
    for i in 0..promo_seg.part.len() {
        insert_into_promo_part(&mut promo_seg.part[i], &p)?;
    }

    Ok(())
}
fn insert_into_promo_part(
    promo_part: &mut promo_json::Part,
    p: &Pas,
) -> Result<(), std::num::ParseIntError> {
    for i in 0..promo_part.type_prod.len() {
        if promo_part.type_prod[i]
            .part_numbers
            .contains(&p.item_name[p.part_number_index].value)
        {
            promo_part.type_prod[i]
                .found_numbers
                .push((*p.item_name).clone());
            promo_part.type_prod[i].total_qty = promo_part.type_prod[i].total_qty
                + p.item_name[p.qty_index].value.parse::<i64>()?;
        }
    }

    Ok(())
}

fn insert_into_promo_helper(
    promo_root: &mut promo_json::Promotion,
    p: &Pas,
) -> Result<(), std::num::ParseIntError> {
    for i in 0..promo_root.promo_sections.len() {
        insert_into_promo_section(&mut promo_root.promo_sections[i], &p)?;
    }
    Ok(())
}
pub fn insert_into_promo(
    promo: &mut HashMap<String, promo_json::Promotion>,
    customer_name: String,
    promo_template: &promo_json::Promotion,
    p: &Pas,
) -> Result<(), Vec<std::num::ParseIntError>> {
    let mut errors: Vec<std::num::ParseIntError> = Vec::new();
    promo
        .entry(customer_name)
        .and_modify(|y| match insert_into_promo_helper(y, &p) {
            Ok(()) => {}
            Err(err) => {
                errors.push(err);
            }
        })
        .or_insert({
            let mut cl = promo_template.clone();
            insert_into_promo_helper(&mut cl, &p);
            cl
        });

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(())
    }
}
