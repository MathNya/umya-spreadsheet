use fancy_regex::Regex;

const ALPHABET: &[&str] = &[
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

pub fn column_index_from_string<S: AsRef<str>>(column: S) -> u32 {
    let column_c = column.as_ref();
    if column_c == "0" {
        return 0;
    }
    match column_c.len() {
        3 => {
            let a = &column_c[0..1];
            let b = &column_c[1..2];
            let c = &column_c[2..3];
            get_index(a) * 676 + get_index(b) * 26 + get_index(c)
        }
        2 => {
            let a = &column_c[0..1];
            let b = &column_c[1..2];
            get_index(a) * 26 + get_index(b)
        }
        1 => get_index(&column_c[0..1]),
        _ => {
            panic!("longer than 3 characters");
        }
    }
}

fn get_index(column: &str) -> u32 {
    self::ALPHABET
        .iter()
        .enumerate()
        .find_map(|(i, tar)| (tar == &column).then(|| i as u32))
        .expect("illegal character")
        + 1
}

pub fn string_from_column_index(column_index: &u32) -> String {
    if column_index < &1u32 {
        panic!("Column number starts from 1.");
    }
    let mut result: String = String::from("");
    let mut index_value = *column_index;
    while index_value > 0 {
        let character_value = (index_value - 1) % 26 + 1;
        index_value = (index_value - character_value) / 26;
        result = format!(
            "{}{}",
            self::ALPHABET.get((character_value - 1) as usize).unwrap(),
            result
        );
    }
    result
}

pub fn coordinate_from_string(coordinate: &str) -> Vec<Option<&str>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\$)?([A-Z]+))?((\$)?([0-9]+))?").unwrap();
    }
    let caps = RE.captures(coordinate).ok().flatten();
    let cols = caps.map(|v| {
        (
            v.get(2).map(|v| v.as_str()),
            v.get(3).map(|v| v.as_str()),
            v.get(5).map(|v| v.as_str()),
            v.get(6).map(|v| v.as_str()),
        )
    });

    let col = cols.map(|v| v.1);
    let is_lock_col = match col.flatten() {
        Some(_) => match cols {
            Some(v) => match v.0.map(|v| v.len()) {
                Some(1..) => Some("1"),
                _ => Some("0"),
            },
            _ => None,
        },
        None => None,
    };

    let row = cols.map(|v| v.3);
    let is_lock_row = match row.flatten() {
        Some(_) => match cols {
            Some(v) => match v.2.map(|v| v.len()) {
                Some(1..) => Some("1"),
                _ => Some("0"),
            },
            _ => None,
        },
        None => None,
    };

    vec![col.flatten(), row.flatten(), is_lock_col, is_lock_row]
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

pub fn index_from_coordinate<S: AsRef<str>>(coordinate: S) -> Vec<Option<u32>> {
    let split = coordinate_from_string(coordinate.as_ref());
    let col = split[0].map(column_index_from_string);
    let row = split[1].map(|v| v.parse::<u32>().unwrap());
    let is_lock_col = split[2].map(|v| v.parse::<u32>().unwrap());
    let is_lock_row = split[3].map(|v| v.parse::<u32>().unwrap());
    vec![col, row, is_lock_col, is_lock_row]
}

pub fn index_from_coordinate_simple(coordinate: &str) -> (u32, u32) {
    let coordinate_upper = coordinate.to_uppercase();
    let split = index_from_coordinate(&coordinate_upper);
    let col = split[0].unwrap();
    let row = split[1].unwrap();
    (col, row)
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
            vec![Some(1), Some(4), Some(1), Some(1)]
        );
        assert_eq!(
            index_from_coordinate("$A4"),
            vec![Some(1), Some(4), Some(1), Some(0)]
        );
        assert_eq!(
            index_from_coordinate("A4"),
            vec![Some(1), Some(4), Some(0), Some(0)]
        );
        assert_eq!(
            index_from_coordinate("Z91"),
            vec![Some(26), Some(91), Some(0), Some(0)]
        );
        assert_eq!(
            index_from_coordinate("AA91"),
            vec![Some(27), Some(91), Some(0), Some(0)]
        );
        assert_eq!(
            index_from_coordinate("AA$91"),
            vec![Some(27), Some(91), Some(0), Some(1)]
        );
        assert_eq!(
            index_from_coordinate("$AA91"),
            vec![Some(27), Some(91), Some(1), Some(0)]
        );
        assert_eq!(
            index_from_coordinate("$AA$91"),
            vec![Some(27), Some(91), Some(1), Some(1)]
        );
        assert_eq!(
            index_from_coordinate("A"),
            vec![Some(1), None, Some(0), None]
        );
        assert_eq!(
            index_from_coordinate("$A"),
            vec![Some(1), None, Some(1), None]
        );
        assert_eq!(
            index_from_coordinate("5"),
            vec![None, Some(5), None, Some(0)]
        );
        assert_eq!(
            index_from_coordinate("$5"),
            vec![None, Some(5), None, Some(1)]
        );
    }
}
