use fancy_regex::{Captures, Regex};
use helper::coordinate::*;

pub fn adjustment_insert_formula_coordinate(
    formula: &str,
    root_col_num: &u32,
    offset_col_num: &u32,
    root_row_num: &u32,
    offset_row_num: &u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
) -> String {
    let re = Regex::new(r#"[^\(]*!*[A-Z]+[0-9]+\:[A-Z]+[0-9]+"#).unwrap();
    let result = re.replace_all(formula, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str().to_string();
        let split_str: Vec<&str> = caps_string.split('!').collect();
        let with_wksheet: bool;
        let wksheet: String;
        let range: String;

        if split_str.len() == 2 {
            with_wksheet = true;
            wksheet = split_str.first().unwrap().to_string();
            range = split_str.get(1).unwrap().to_string();
        } else {
            with_wksheet = false;
            wksheet = self_worksheet_name.to_string();
            range = split_str.first().unwrap().to_string();
        }

        if &wksheet != &worksheet_name {
            return caps_string;
        }

        let split_range: Vec<&str> = range.split(':').collect();
        let mut result = String::from("");

        for coordinate in split_range {
            let index_coordinate = index_from_coordinate(coordinate);
            let is_lock_col = index_coordinate.2.unwrap();
            let is_lock_row = index_coordinate.3.unwrap();
            let col_num = adjustment_insert_coordinate(
                &index_coordinate.0.unwrap(),
                root_col_num,
                offset_col_num,
            );
            let row_num = adjustment_insert_coordinate(
                &index_coordinate.1.unwrap(),
                root_row_num,
                offset_row_num,
            );
            let new_corrdinate =
                coordinate_from_index_with_lock(&col_num, &row_num, &is_lock_col, &is_lock_row);

            if !&result.is_empty() {
                result = format!("{}:", result);
            }
            result = format!("{}{}", result, new_corrdinate);
        }

        if with_wksheet {
            result = format!("{}!{}", wksheet, result);
        }

        result
    });

    result.into()
}

pub fn adjustment_remove_formula_coordinate(
    formula: &str,
    root_col_num: &u32,
    offset_col_num: &u32,
    root_row_num: &u32,
    offset_row_num: &u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
) -> String {
    let re = Regex::new(r#"[^\(]*!*[A-Z]+[0-9]+\:[A-Z]+[0-9]+"#).unwrap();
    let result = re.replace_all(formula, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str().to_string();
        let split_str: Vec<&str> = caps_string.split('!').collect();
        let with_wksheet: bool;
        let wksheet: String;
        let range: String;

        if split_str.len() == 2 {
            with_wksheet = true;
            wksheet = split_str.first().unwrap().to_string();
            range = split_str.get(1).unwrap().to_string();
        } else {
            with_wksheet = false;
            wksheet = self_worksheet_name.to_string();
            range = split_str.first().unwrap().to_string();
        }

        if &wksheet != &worksheet_name {
            return caps_string;
        }

        let split_range: Vec<&str> = range.split(':').collect();
        let mut result = String::from("");

        for coordinate in split_range {
            let index_coordinate = index_from_coordinate(coordinate);
            let is_lock_col = index_coordinate.2.unwrap();
            let is_lock_row = index_coordinate.3.unwrap();
            let col_num = adjustment_remove_coordinate(
                &index_coordinate.0.unwrap(),
                root_col_num,
                offset_col_num,
            );
            let row_num = adjustment_remove_coordinate(
                &index_coordinate.1.unwrap(),
                root_row_num,
                offset_row_num,
            );
            let new_corrdinate =
                coordinate_from_index_with_lock(&col_num, &row_num, &is_lock_col, &is_lock_row);

            if !&result.is_empty() {
                result = format!("{}:", result);
            }

            result = format!("{}{}", result, new_corrdinate);
        }

        if with_wksheet {
            result = format!("{}!{}", wksheet, result);
        }

        result
    });

    result.into()
}
