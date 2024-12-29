use std::iter::successors;

use crate::helper::utils::compile_regex;

/// Converts a 1-based index to a string representation using letters
/// similar to Excel column naming (e.g., 1 -> "A", 27 -> "AA").
///
/// # Arguments
///
/// * `index` - A 1-based index to convert to a string. Must be greater than or
///   equal to 1.
///
/// # Returns
///
/// A `String` representing the column name corresponding to the given index.
///
/// # Panics
///
/// Panics if the `index` is less than 1.
fn index_to_alpha(index: u32) -> String {
    const BASE_CHAR_CODE: u32 = 'A' as u32; // char 'A'

    assert!(index >= 1, "Index cannot be less than one.");

    // Generate the sequence of characters for the given index
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

/// Converts a string representation of a column name (e.g., "A", "AA")
/// to its corresponding 1-based index.
///
/// # Arguments
///
/// * `alpha` - A string slice or reference to a string representing the column
///   name.
///
/// # Returns
///
/// A `u32` representing the 1-based index corresponding to the given column
/// name.
///
/// # Examples
///
/// ```
/// let index = alpha_to_index("AA");
/// assert_eq!(index, 27);
/// ```
fn alpha_to_index<S>(alpha: S) -> u32
where
    S: AsRef<str>,
{
    const BASE_CHAR_CODE: u32 = 'A' as u32;
    // Pre-computed powers of 26 for up to three characters
    const POSITIONAL_CONSTANTS: [u32; 3] = [1, 26, 676];

    alpha
        .as_ref()
        .chars()
        .rev()
        .enumerate()
        .map(|(index, v)| {
            let vn = (v as u32 - BASE_CHAR_CODE) + 1;

            POSITIONAL_CONSTANTS[index] * vn
        })
        .sum::<u32>()
}

/// Converts a column name string to its corresponding 1-based index,
/// returning 0 if the input is "0".
///
/// # Arguments
///
/// * `column` - A string slice or reference to a string representing the column
///   name.
///
/// # Returns
///
/// A `u32` representing the 1-based index corresponding to the given column
/// name, or 0 if the input is "0".
///
/// # Examples
///
/// ```
/// let index = column_index_from_string("AA");
/// assert_eq!(index, 27);
///
/// let index_zero = column_index_from_string("0");
/// assert_eq!(index_zero, 0);
/// ```
pub fn column_index_from_string<S: AsRef<str>>(column: S) -> u32 {
    let column_c = column.as_ref();
    if column_c == "0" {
        return 0;
    }

    alpha_to_index(column_c)
}

/// Converts a 1-based column index to its corresponding Excel-style column
/// label.
///
/// This function takes a column index starting from 1 and returns a string
/// representing the column label as used in Excel (e.g., 1 -> "A", 27 -> "AA").
///
/// # Parameters
///
/// - `column_index`: A 1-based column index. Must be greater than or equal to
///   1.
///
/// # Returns
///
/// A `String` representing the Excel-style column label.
///
/// # Panics
///
/// Panics if the `column_index` is less than 1, as column numbering starts from
/// 1.
///
/// # Examples
///
/// ```
/// let label = string_from_column_index(1);
/// assert_eq!(label, "A");
///
/// let label = string_from_column_index(28);
/// assert_eq!(label, "AB");
/// ```
#[must_use]
pub fn string_from_column_index(column_index: u32) -> String {
    assert!(column_index >= 1u32, "Column number starts from 1.");

    index_to_alpha(column_index)
}

/// Parses an Excel-style coordinate string into column and row indices, along
/// with lock flags.
///
/// This function takes a coordinate string, which may include column letters
/// and row numbers, and optional '$' symbols indicating locked columns or rows.
/// It returns a tuple containing the column index, row index, and optional lock
/// flags for each.
///
/// # Type Parameters
///
/// - `T`: A type that can be referenced as a string slice, such as `&str` or
///   `String`.
///
/// # Parameters
///
/// - `coordinate`: The coordinate string to parse. It should be in the format
///   of Excel cell references, such as "A1", "$B$2", or "C$3".
///
/// # Returns
///
/// A tuple `(Option<u32>, Option<u32>, Option<bool>, Option<bool>)` where:
/// - The first element is the column index, if present, calculated using
///   `alpha_to_index`.
/// - The second element is the row index, if present, parsed as a `u32`.
/// - The third element is a flag indicating if the column is locked, if
///   applicable.
/// - The fourth element is a flag indicating if the row is locked, if
///   applicable.
///
/// # Examples
///
/// ```
/// let (col, row, col_lock, row_lock) = index_from_coordinate("$AB$12");
/// assert_eq!(col, Some(28)); // 'AB' corresponds to column index 28
/// assert_eq!(row, Some(12));
/// assert_eq!(col_lock, Some(true));
/// assert_eq!(row_lock, Some(true));
/// ```
///
/// # Panics
///
/// This function does not panic. It returns `None` for column or row indices if
/// they cannot be parsed.
pub fn index_from_coordinate<T>(coordinate: T) -> CellIndex
where
    T: AsRef<str>,
{
    let re = compile_regex!(r"((\$)?([A-Z]{1,3}))?((\$)?([0-9]+))?");

    re.captures(coordinate.as_ref())
        .ok()
        .flatten()
        .map(|v| {
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

/// Converts a column index and row index into an Excel-style coordinate string.
///
/// This function takes a 1-based column index and a row index, and returns a
/// string representing the coordinate in Excel format (e.g., (1, 1) -> "A1").
///
/// # Parameters
///
/// - `col`: A 1-based column index. Must be greater than or equal to 1.
/// - `row`: A 1-based row index. Must be greater than or equal to 1.
///
/// # Returns
///
/// A `String` representing the Excel-style coordinate.
///
/// # Examples
///
/// ```
/// let coordinate = coordinate_from_index(1, 1);
/// assert_eq!(coordinate, "A1");
///
/// let coordinate = coordinate_from_index(28, 3);
/// assert_eq!(coordinate, "AB3");
/// ```
#[must_use]
pub fn coordinate_from_index(col: u32, row: u32) -> String {
    format!("{}{}", string_from_column_index(col), row)
}

#[must_use]
pub fn coordinate_from_index_with_lock(
    col: u32,
    row: u32,
    is_lock_col: bool,
    is_lock_row: bool,
) -> String {
    format!(
        "{}{}{}{}",
        if is_lock_col { "$" } else { "" },
        string_from_column_index(col),
        if is_lock_row { "$" } else { "" },
        row
    )
}

/// Adjusts a coordinate index by inserting an offset if conditions are met.
///
/// This function checks if the given `num` is greater than or equal to
/// `root_num` and if `offset_num` is not zero. If both conditions are
/// satisfied, it returns the sum of `num` and `offset_num`. Otherwise, it
/// returns `num` unchanged.
///
/// # Parameters
///
/// - `num`: The original coordinate index to be adjusted.
/// - `root_num`: The threshold coordinate index for applying the offset.
/// - `offset_num`: The offset to be added to `num` if conditions are met.
///
/// # Returns
///
/// A `u32` representing the adjusted coordinate index.
///
/// # Examples
///
/// ```
/// let adjusted = adjustment_insert_coordinate(5, 3, 2);
/// assert_eq!(adjusted, 7); // 5 + 2 since 5 >= 3 and offset is not zero
///
/// let adjusted = adjustment_insert_coordinate(2, 3, 2);
/// assert_eq!(adjusted, 2); // No adjustment since 2 < 3
/// ```
pub(crate) fn adjustment_insert_coordinate(num: u32, root_num: u32, offset_num: u32) -> u32 {
    if num >= root_num && offset_num != 0 {
        num + offset_num
    } else {
        num
    }
}

/// Adjusts a coordinate index by removing an offset if conditions are met.
///
/// This function checks if the given `num` is greater than or equal to
/// `root_num` and if `offset_num` is not zero. If both conditions are
/// satisfied, it returns the result of subtracting `offset_num` from `num`.
/// Otherwise, it returns `num` unchanged.
///
/// # Parameters
///
/// - `num`: The original coordinate index to be adjusted.
/// - `root_num`: The threshold coordinate index for applying the offset.
/// - `offset_num`: The offset to be subtracted from `num` if conditions are
///   met.
///
/// # Returns
///
/// A `u32` representing the adjusted coordinate index.
///
/// # Examples
///
/// ```
/// let adjusted = adjustment_remove_coordinate(5, 3, 2);
/// assert_eq!(adjusted, 3); // 5 - 2 since 5 >= 3 and offset is not zero
///
/// let adjusted = adjustment_remove_coordinate(2, 3, 2);
/// assert_eq!(adjusted, 2); // No adjustment since 2 < 3
/// ```
pub(crate) fn adjustment_remove_coordinate(num: u32, root_num: u32, offset_num: u32) -> u32 {
    if num >= root_num && offset_num != 0 {
        num - offset_num
    } else {
        num
    }
}

/// Determines if a coordinate index falls within a removable range.
///
/// This function checks whether the given `num` is within the range defined by
/// `root_num` and `offset_num`. Specifically, it returns `true` if `num` is
/// greater than or equal to `root_num` and less than the sum of `root_num` and
/// `offset_num`, provided both `root_num` and `offset_num` are non-zero.
///
/// # Parameters
///
/// - `num`: The coordinate index to check.
/// - `root_num`: The starting index of the range.
/// - `offset_num`: The length of the range to be checked.
///
/// # Returns
///
/// A `bool` indicating whether `num` is within the removable range.
///
/// # Examples
///
/// ```
/// let is_removable = is_remove_coordinate(5, 3, 2);
/// assert_eq!(is_removable, true); // 5 is within the range [3, 5)
///
/// let is_removable = is_remove_coordinate(6, 3, 2);
/// assert_eq!(is_removable, false); // 6 is outside the range [3, 5)
/// ```
pub(crate) fn is_remove_coordinate(num: u32, root_num: u32, offset_num: u32) -> bool {
    if root_num != 0 && offset_num != 0 {
        return num >= root_num && num < (root_num + offset_num);
    }
    false
}

/// Type alias for a tuple representing cell index information.
///
/// This tuple contains:
/// - An optional column index (`Option<u32>`).
/// - An optional row index (`Option<u32>`).
/// - An optional column lock flag (`Option<bool>`).
/// - An optional row lock flag (`Option<bool>`).
pub type CellIndex = (Option<u32>, Option<u32>, Option<bool>, Option<bool>);

/// Struct for representing cell coordinates with row and column numbers.
#[derive(Clone, Debug)]
pub struct CellCoordinates {
    pub row: u32, // The 1-based row index of the cell.
    pub col: u32, // The 1-based column index of the cell.
}

impl CellCoordinates {
    /// Creates a new `CellCoordinates` instance with the specified column and
    /// row indices.
    ///
    /// # Parameters
    ///
    /// - `col`: The 1-based column index.
    /// - `row`: The 1-based row index.
    ///
    /// # Returns
    ///
    /// A new `CellCoordinates` instance.
    fn new(col: u32, row: u32) -> Self {
        CellCoordinates { row, col }
    }
}

impl From<(u32, u32)> for CellCoordinates {
    /// Converts a tuple of column and row indices into a `CellCoordinates`
    /// instance.
    ///
    /// # Parameters
    ///
    /// - `value`: A tuple containing the 1-based column and row indices.
    ///
    /// # Returns
    ///
    /// A `CellCoordinates` instance representing the specified indices.
    fn from(value: (u32, u32)) -> Self {
        CellCoordinates::new(value.0, value.1)
    }
}

impl From<String> for CellCoordinates {
    /// Converts a string representation of a cell coordinate into a
    /// `CellCoordinates` instance.
    ///
    /// The string is expected to be in Excel-style format (e.g., "A1").
    ///
    /// # Parameters
    ///
    /// - `value`: A string containing the cell coordinate.
    ///
    /// # Returns
    ///
    /// A `CellCoordinates` instance representing the specified coordinate.
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for CellCoordinates {
    /// Converts a string slice representation of a cell coordinate into a
    /// `CellCoordinates` instance.
    ///
    /// The string is expected to be in Excel-style format (e.g., "A1"). The
    /// function converts the string to uppercase before parsing.
    ///
    /// # Parameters
    ///
    /// - `value`: A string slice containing the cell coordinate.
    ///
    /// # Returns
    ///
    /// A `CellCoordinates` instance representing the specified coordinate.
    fn from(value: &str) -> Self {
        let (col, row, ..) = index_from_coordinate(value.to_uppercase());
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
        assert_eq!(column_index_from_string("XFD"), 16384); // Max. supported by
        // Excel 2012
    }

    #[test]
    fn string_from_column_index_1() {
        assert_eq!(string_from_column_index(1), String::from("A"));
        assert_eq!(string_from_column_index(26), String::from("Z"));
        assert_eq!(string_from_column_index(27), String::from("AA"));
        assert_eq!(string_from_column_index(28), String::from("AB"));
        assert_eq!(string_from_column_index(53), String::from("BA"));
        assert_eq!(string_from_column_index(702), String::from("ZZ"));
        assert_eq!(string_from_column_index(703), String::from("AAA"));
        assert_eq!(string_from_column_index(8160), String::from("LAV"));
        assert_eq!(string_from_column_index(16384), String::from("XFD"));
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
