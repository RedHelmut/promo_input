use std::collections::HashMap;


use crate::general::promo_json::{PromoSection, Part, Promotion};
use crate::general::and_or::AndOrType;

fn modify_promo_section(promo_seg: &mut PromoSection) -> Result<(), std::num::ParseIntError> {
    let section_qualified = true;
    for i in 0..promo_seg.part.len() {
        modify_promo_part(&mut promo_seg.part[i])?;
    }

    let min_qualified: i64 = match promo_seg
        .part
        .iter_mut()
        .min_by_key(|k| k.times_parts_qualified)
    {
        Some(valid) => valid.times_parts_qualified,
        None => {
            //empty iterator so 0
            0
        }
    };

    for i in 0..promo_seg.part.len() {
        if promo_seg.part[i].times_parts_qualified == min_qualified {
            promo_seg.promo_parts_still_needed.push(i);
        }
    }

    if section_qualified {
        match promo_seg
            .part
            .iter_mut()
            .min_by_key(|x| x.times_parts_qualified)
        {
            Some(a) => {
                promo_seg.times_section_qualified = a.times_parts_qualified;
            }
            None => {}
        }
    }
    Ok(())
}
fn modify_promo_part(promo_part: &mut Part) -> Result<(), std::num::ParseIntError> {
    for i in 0..promo_part.type_prod.len() {
        promo_part.type_prod[i].times_qualified = (promo_part.type_prod[i].total_qty as f64
            / promo_part.type_prod[i].qty_needed)
            .floor() as i64;
    }

    match promo_part.part_type {
        AndOrType::And => {
            let min_qualified: i64 = match promo_part
                .type_prod
                .iter_mut()
                .min_by_key(|k| k.times_qualified)
            {
                Some(valid) => valid.times_qualified,
                None => {
                    //empty iterator so 0
                    0
                }
            };
            promo_part.type_prods_for_next_promo_needed = Vec::new();

            for i in 0..promo_part.type_prod.len() {
                if promo_part.type_prod[i].times_qualified == min_qualified {
                    promo_part.type_prods_for_next_promo_needed.push(i as i64);
                }
            }

            if min_qualified > 0 {
                promo_part.times_parts_qualified = min_qualified;
            }
        }
        AndOrType::Or => {
            promo_part.type_prods_for_next_promo_needed = Vec::new();
            promo_part.times_parts_qualified = promo_part
                .type_prod
                .iter_mut()
                .map(|x| x.times_qualified)
                .sum();

            for i in 0..promo_part.type_prod.len() {
                promo_part.type_prods_for_next_promo_needed.push(i as i64);
            }
        }
        AndOrType::Any(cnt) => {
            promo_part.type_prods_for_next_promo_needed = Vec::new();
            promo_part.times_parts_qualified = (promo_part
                .type_prod
                .iter_mut()
                .map(|x| x.times_qualified)
                .sum::<i64>() as f64
                / cnt as f64)
                .floor() as i64;
            for i in 0..promo_part.type_prod.len() {
                promo_part.type_prods_for_next_promo_needed.push(i as i64);
            }
        }
        AndOrType::None => {
            if promo_part.type_prod.len() == 1 {
                promo_part.type_prods_for_next_promo_needed = Vec::new();
                promo_part.times_parts_qualified = promo_part.type_prod[0].times_qualified;
                promo_part.type_prods_for_next_promo_needed.push(0);
            } else {
                //should be an error
                "Apple Jacks".parse::<i64>()?;
            }
        }
    }

    Ok(())
}

fn modify_promo_helper(promo_root: &mut Promotion) -> Result<(), std::num::ParseIntError> {
    for i in 0..promo_root.promo_sections.len() {
        modify_promo_section(&mut promo_root.promo_sections[i])?;
    }
    Ok(())
}

pub fn complete_promo(
    promo: &mut HashMap<String, Promotion>,
) -> Result<(), Vec<std::num::ParseIntError>> {
    let mut errors: Vec<std::num::ParseIntError> = Vec::new();
    for i in promo.values_mut() {
        match modify_promo_helper(i) {
            Ok(()) => {}
            Err(err) => {
                errors.push(err);
            }
        }
    }
    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(())
    }
}
