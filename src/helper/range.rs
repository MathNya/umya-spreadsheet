use helper::coordinate::*;

pub fn get_coordinate_list(range_str: &str) -> Vec<(u32, u32)> {
    let coordinate_collection: Vec<&str> = range_str.split(':').collect();
    if coordinate_collection.is_empty() || coordinate_collection.len() > 2 {
        panic!("Non-standard range.");
    }

    let mut result: Vec<(u32, u32)> = Vec::new();

    let mut is_col_select = false;
    let mut is_row_select = false;
    let mut col_start = 0;
    let mut col_end = 0;
    let mut row_start = 0;
    let mut row_end = 0;

    if coordinate_collection.len() == 1 || coordinate_collection.len() == 2 {
        let coordinate_str = coordinate_collection[0].to_string();
        let nums = index_from_coordinate(coordinate_str);
        match nums[0] {
            Some(v) => {
                is_col_select = true;
                col_start = v;
                col_end = v;
            }
            None => {}
        };
        match nums[1] {
            Some(v) => {
                is_row_select = true;
                row_start = v;
                row_end = v;
            }
            None => {}
        }
    }

    if coordinate_collection.len() == 2 {
        let coordinate_str = coordinate_collection[1].to_string();
        let nums = index_from_coordinate(coordinate_str);
        match nums[0] {
            Some(v) => {
                col_end = v;
            }
            None => {
                if !is_col_select {
                    panic!("Non-standard range.");
                }
            }
        };
        match nums[1] {
            Some(v) => {
                row_end = v;
            }
            None => {
                if !is_row_select {
                    panic!("Non-standard range.");
                }
            }
        }
    }

    for row_num in row_start..=row_end {
        for col_num in col_start..=col_end {
            result.push((col_num, row_num));
        }
    }

    result
}
