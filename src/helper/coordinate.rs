use std::iter::successors;

use fancy_regex::Regex;

fn index_to_alpha(index: u32) -> String {
    if index < 1 {
        panic!("Index cannot be less than one.")
    }

    const BASE_CHAR_CODE: u32 = 'A' as u32;
    // below code is based on the source code of `radix_fmt`
    successors(Some(index - 1), |index| match index / 26u32 {
        0 => None,
        n => Some(n - 1),
    })
    .map(|v| BASE_CHAR_CODE + (v % 26))
    .collect::<Vec<u32>>()
    .into_iter()
    .rev()
    .map(|v| char::from_u32(v).unwrap())
    .collect()
}

fn alpha_to_index<S>(alpha: S) -> u32
where
    S: AsRef<str>,
{
    const BASE_CHAR_CODE: u32 = 'A' as u32;
    // since we only allow up to three characters, we can use pre-computed
    /// powers of 26 `[26^0, 26^1, 26^2]`
    const POSITIONAL_CONSTANTS: [u32; 3] = [1, 26, 676];

    alpha
        .as_ref()
        .chars()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, v)| {
            let vn = (v as u32 - BASE_CHAR_CODE) + 1;

            // 26u32.pow(index as u32) * vn
            POSITIONAL_CONSTANTS[index] * vn
        })
        .sum::<u32>()
}

pub fn column_index_from_string<S: AsRef<str>>(column: S) -> u32 {
    let column_c = column.as_ref();
    if column_c == "0" {
        return 0;
    }

    alpha_to_index(column_c)
}

pub fn string_from_column_index(column_index: &u32) -> String {
    if column_index < &1u32 {
        panic!("Column number starts from 1.");
    }

    index_to_alpha(*column_index)
}

///
/// # Returns
/// A tuple with the column, and row address indexes and their respective lock flags.
/// <br />
/// i.e. `(col, row, col_lock_flg, row_lock_flg)`
/// ## Note:
/// The minimum value for `col` and `row` is 1
pub fn index_from_coordinate<T>(coordinate: T) -> CellIndex
where
    T: AsRef<str>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\$)?([A-Z]{1,3}))?((\$)?([0-9]+))?").unwrap();
    }

    let caps = RE.captures(coordinate.as_ref()).ok().flatten();

    caps.map(|v| {
        let col = v.get(3).map(|v| alpha_to_index(v.as_str())); // col number: [A-Z]{1,3}
        let row = v.get(6).and_then(|v| v.as_str().parse::<u32>().ok()); // row number: [0-9]+

        let col_lock_flg = col.map(|_col| {
            v.get(2).is_some() // col lock flag: (\$)?
        });

        let row_lock_flg = row.map(|_row| {
            v.get(5).is_some() // row lock flag: (\$)?
        });

        (col, row, col_lock_flg, row_lock_flg)
    })
    .unwrap_or_default()
}

pub fn coordinate_from_index(col: &u32, row: &u32) -> String {
    format!("{}{}", string_from_column_index(col), row)
}

pub fn coordinate_from_index_with_lock(
    col: &u32,
    row: &u32,
    is_lock_col: &bool,
    is_lock_row: &bool,
) -> String {
    format!(
        "{}{}{}{}",
        if is_lock_col == &true { "$" } else { "" },
        string_from_column_index(col),
        if is_lock_row == &true { "$" } else { "" },
        row
    )
}

#[deprecated(note = "use `CellCoordinates::from` instead")]
pub fn index_from_coordinate_simple(coordinate: &str) -> (u32, u32) {
    let coordinate_upper = coordinate.to_uppercase();
    let (col, row, ..) = index_from_coordinate(coordinate_upper);
    (col.unwrap(), row.unwrap())
}

pub(crate) fn adjustment_insert_coordinate(num: &u32, root_num: &u32, offset_num: &u32) -> u32 {
    let mut result = *num;
    if (num >= root_num && offset_num > &0) || (num < root_num && offset_num < &0) {
        result += offset_num;
    }
    result
}

pub(crate) fn adjustment_remove_coordinate(num: &u32, root_num: &u32, offset_num: &u32) -> u32 {
    let mut result = *num;
    if (num >= root_num && offset_num > &0) || (num < root_num && offset_num < &0) {
        result -= offset_num;
    }
    result
}

pub type CellIndex = (Option<u32>, Option<u32>, Option<bool>, Option<bool>);

/// Struct for representing cell coordinates with row and column numbers
pub struct CellCoordinates {
    pub row: u32,
    pub col: u32,
}

impl CellCoordinates {
    fn new(col: u32, row: u32) -> Self {
        CellCoordinates { row, col }
    }
}

impl From<(u32, u32)> for CellCoordinates {
    fn from(value: (u32, u32)) -> Self {
        CellCoordinates::new(value.0, value.1)
    }
}

impl From<(&u32, &u32)> for CellCoordinates {
    fn from(value: (&u32, &u32)) -> Self {
        CellCoordinates::new(*value.0, *value.1)
    }
}

impl From<String> for CellCoordinates {
    fn from(value: String) -> Self {
        let str_ref: &str = value.as_ref();
        str_ref.into()
    }
}

impl From<&str> for CellCoordinates {
    fn from(value: &str) -> Self {
        let coordinate_upper = value.to_uppercase();
        let (col, row, ..) = index_from_coordinate(coordinate_upper);
        CellCoordinates::new(col.unwrap(), row.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_index_from_string_1() {
        assert_eq!(column_index_from_string("A"), 1);
        assert_eq!(column_index_from_string("B"), 2);
        assert_eq!(column_index_from_string("Z"), 26);
        assert_eq!(column_index_from_string("AA"), 27);
        assert_eq!(column_index_from_string("AB"), 28);
        assert_eq!(column_index_from_string("BA"), 53);
        assert_eq!(column_index_from_string("ZZ"), 702);
        assert_eq!(column_index_from_string("AAA"), 703);
        assert_eq!(column_index_from_string("LAV"), 8160);
        assert_eq!(column_index_from_string("XFD"), 16384); // Max. supported by Excel 2102
    }

    #[test]
    fn string_from_column_index_1() {
        assert_eq!(string_from_column_index(&1), String::from("A"));
        assert_eq!(string_from_column_index(&26), String::from("Z"));
        assert_eq!(string_from_column_index(&27), String::from("AA"));
        assert_eq!(string_from_column_index(&28), String::from("AB"));
        assert_eq!(string_from_column_index(&53), String::from("BA"));
        assert_eq!(string_from_column_index(&702), String::from("ZZ"));
        assert_eq!(string_from_column_index(&703), String::from("AAA"));
        assert_eq!(string_from_column_index(&8160), String::from("LAV"));
        assert_eq!(string_from_column_index(&16384), String::from("XFD"));
    }

    #[test]
    fn index_from_coordinate_1() {
        assert_eq!(
            index_from_coordinate("$A$4"),
            (Some(1), Some(4), Some(true), Some(true))
        );
        assert_eq!(
            index_from_coordinate("$A4"),
            (Some(1), Some(4), Some(true), Some(false))
        );
        assert_eq!(
            index_from_coordinate("A4"),
            (Some(1), Some(4), Some(false), Some(false))
        );
        assert_eq!(
            index_from_coordinate("Z91"),
            (Some(26), Some(91), Some(false), Some(false))
        );
        assert_eq!(
            index_from_coordinate("AA91"),
            (Some(27), Some(91), Some(false), Some(false))
        );
        assert_eq!(
            index_from_coordinate("AA$91"),
            (Some(27), Some(91), Some(false), Some(true))
        );
        assert_eq!(
            index_from_coordinate("$AA91"),
            (Some(27), Some(91), Some(true), Some(false))
        );
        assert_eq!(
            index_from_coordinate("$AA$91"),
            (Some(27), Some(91), Some(true), Some(true))
        );
        assert_eq!(
            index_from_coordinate("A"),
            (Some(1), None, Some(false), None)
        );
        assert_eq!(
            index_from_coordinate("$A"),
            (Some(1), None, Some(true), None)
        );
        assert_eq!(
            index_from_coordinate("5"),
            (None, Some(5), None, Some(false))
        );
        assert_eq!(
            index_from_coordinate("$5"),
            (None, Some(5), None, Some(true))
        );
    }
}
