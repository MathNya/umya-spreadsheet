use crate::helper::coordinate::index_from_coordinate;

/// `(col, row)`
pub type BasicCellIndex = (u32, u32);

/// # Returns
/// `Vec<(col, row)>`
#[must_use]
pub fn get_coordinate_list(range_str: &str) -> Vec<BasicCellIndex> {
    let (row_start, row_end, col_start, col_end) = get_start_and_end_point(range_str);

    (row_start..=row_end)
        .flat_map(|row_num| (col_start..=col_end).map(move |col_num| (col_num, row_num)))
        .collect()
}

#[must_use]
pub fn get_start_and_end_point(range_str: &str) -> (u32, u32, u32, u32) {
    let coordinate_collection: Vec<&str> = range_str.split(':').collect();

    assert!(
        matches!(coordinate_collection.len(), 1 | 2),
        "Non-standard range."
    );

    let mut is_col_select = false;
    let mut is_row_select = false;
    let (mut col_start, mut col_end, mut row_start, mut row_end) = (0, 0, 0, 0);

    let (col, row, ..) = index_from_coordinate(coordinate_collection[0]);

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

    if coordinate_collection.len() == 2 {
        let (col, row, ..) = index_from_coordinate(coordinate_collection[1]);

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

#[inline]
#[must_use]
pub fn get_split_range(range: &str) -> Vec<&str> {
    range.split(':').collect()
}

#[inline]
#[must_use]
pub fn get_join_range(coordinate_list: &[String]) -> String {
    coordinate_list.join(":")
}
