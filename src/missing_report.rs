use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use super::general::and_or::AndOrType;
use super::general::promo_json::{PromoSection, Promotion};

fn get_amount_missing_for_next_promo(qty_needed: i64, qty_claimed: i64) -> i64 {
    if qty_claimed < qty_needed {
        qty_needed - qty_claimed
    } else if qty_claimed == qty_needed {
        qty_needed
    } else {
        // 6
        let promo_count_ceiling = qty_claimed / qty_needed + 1;
        let remaining_needed = promo_count_ceiling * qty_needed - qty_claimed;
        remaining_needed
    }
}

struct MissingPartNumber {
    pub missing_part_numbers: Vec<String>,
    pub amount_needed: i64,
}
struct NeededSections {
    pub missing_part_numbers: Vec<(AndOrType, Vec<MissingPartNumber>)>,
}

fn generate_missing_report_for_section(promo_section: &PromoSection) -> Vec<NeededSections> {
    let mut rv: Vec<NeededSections> = Vec::new();
    for stl_nd_sec_ind in 0..promo_section.promo_parts_still_needed.len() {
        let part_index: usize = promo_section.promo_parts_still_needed[stl_nd_sec_ind];
        let promo_section_part = &promo_section.part[part_index];
        let mut sec = NeededSections {
            missing_part_numbers: Vec::new(),
        };
        let mut missing_pn: Vec<MissingPartNumber> = Vec::new();
        for tpn_index in 0..promo_section_part.type_prods_for_next_promo_needed.len() {
            let total_qty = promo_section_part.type_prod
                [promo_section_part.type_prods_for_next_promo_needed[tpn_index] as usize]
                .total_qty;
            let qty_needed = promo_section_part.type_prod
                [promo_section_part.type_prods_for_next_promo_needed[tpn_index] as usize]
                .qty_needed as i64;
            let rem = get_amount_missing_for_next_promo(qty_needed, total_qty);
            let missing = MissingPartNumber {
                missing_part_numbers: promo_section_part.type_prod
                    [promo_section_part.type_prods_for_next_promo_needed[tpn_index] as usize]
                    .part_numbers
                    .clone(),
                amount_needed: rem,
            };
            missing_pn.push(missing);
        }
        sec.missing_part_numbers
            .push((promo_section_part.part_type.clone(), missing_pn));
        //
        rv.push(sec);
    }
    rv
}

pub fn display_missing_report<W: Write>(
    hsh: &mut HashMap<String, Promotion>,
    write_to: &mut W,
) -> Result<(), std::io::Error> {
    let mut cust_names: Vec<_> = hsh.iter().map(|x| x.0).collect();
    cust_names.sort();
    for name in cust_names {
        write_to.write(format!("For Customer: {}\r\n", name).as_bytes())?;

        for sec_id in 0..hsh[name].promo_sections.len() {
            write_to.write(
                format!(
                    "Qualified {} times for Promo {}\r\n",
                    &hsh[name].promo_sections[sec_id].times_section_qualified,
                    sec_id + 1
                )
                .as_bytes(),
            )?;

            if hsh[name].promo_sections[sec_id].times_section_qualified == 0 {
                write_to.write(format!("To get the promo you need to purchase:\r\n").as_bytes())?;
            } else {
                write_to.write(format!("To get another you need to purchase:\r\n").as_bytes())?;
            }
            let missing_section_data =
                generate_missing_report_for_section(&hsh[name].promo_sections[sec_id]);
            write_missing_report(&missing_section_data, write_to)?;
        }
        write_to.write(format!("\r\n").as_bytes())?;
    }
    Ok(())
}

fn write_missing_report<W: Write>(
    missing_report: &Vec<NeededSections>,
    write_to: &mut W,
) -> Result<(), std::io::Error> {
    for sec_index in 0..missing_report.len() {
        let sec = &missing_report[sec_index];
        for missing_index in 0..sec.missing_part_numbers.len() {
            let (join_type, items) = &sec.missing_part_numbers[missing_index];
            for item_idx in 0..items.len() {
                write_to.write(format!("{} of a ( ", items[item_idx].amount_needed).as_bytes())?;

                display_vec_of_strings_as_csv(&items[item_idx].missing_part_numbers, write_to)?;
                write_to.write(format!(" )").as_bytes())?;

                if item_idx < items.len() - 1 {
                    if join_type == &AndOrType::Or {
                        write_to.write(format!(" {} ", join_type).as_bytes())?;
                    } else {
                        write_to.write(format!("\r\n").as_bytes())?;
                    }
                } else {
                    write_to.write(format!("\r\n").as_bytes())?;
                }
            }
        }
    }
    Ok(())
}

fn display_vec_of_strings_as_csv<W: Write>(
    input: &Vec<String>,
    write_to: &mut W,
) -> Result<(), std::io::Error> {
    for val_idx in 0..input.len() {
        if val_idx < input.len() - 1 {
            let t = input[val_idx].clone() + " ,";
            write_to.write(t.as_bytes())?;
        } else {
            write_to.write(input[val_idx].as_bytes())?;
        }
    }
    Ok(())
}
