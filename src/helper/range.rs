use helper::coordinate::*;

/// `(col, row)`
pub type BasicCellIndex = (u32, u32);

/// # Returns
/// `Vec<(col, row)>`
pub fn get_coordinate_list(range_str: &str) -> Vec<BasicCellIndex> {
    let mut result: Vec<(u32, u32)> = Vec::new();

    let (row_start, row_end, col_start, col_end) = get_start_and_end_point(range_str);
    for row_num in row_start..=row_end {
        for col_num in col_start..=col_end {
            result.push((col_num, row_num));
        }
    }

    result
}

pub fn get_start_and_end_point(range_str: &str) -> (u32, u32, u32, u32) {
    let coordinate_collection: Vec<&str> = range_str.split(':').collect();

    assert!(
        matches!(coordinate_collection.len(), 1 | 2),
        "Non-standard range."
    );

    let mut is_col_select = false;
    let mut is_row_select = false;
    let mut col_start = 0;
    let mut col_end = 0;
    let mut row_start = 0;
    let mut row_end = 0;

    if coordinate_collection.len() == 1 || coordinate_collection.len() == 2 {
        let coordinate_str = coordinate_collection[0].to_string();
        let (col, row, ..) = index_from_coordinate(coordinate_str);

        if let Some(v) = col {
            is_col_select = true;
            col_start = v;
            col_end = v;
        }

        if let Some(v) = row {
            is_row_select = true;
            row_start = v;
            row_end = v;
        }
    }

    if coordinate_collection.len() == 2 {
        let coordinate_str = coordinate_collection[1].to_string();
        let (col, row, ..) = index_from_coordinate(coordinate_str);

        match col {
            Some(v) => {
                col_end = v;
            }
            None => {
                assert!(is_col_select, "Non-standard range.");
            }
        };

        match row {
            Some(v) => {
                row_end = v;
            }
            None => {
                assert!(is_row_select, "Non-standard range.");
            }
        }
    }
    (row_start, row_end, col_start, col_end)
}
