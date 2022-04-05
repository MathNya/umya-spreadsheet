use regex::Regex;

const ALPHABET: &'static [&'static str] = &[
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
            get_index(a) * 676
                + get_index(b) * 26
                + get_index(c)
        }
        2 => {
            let a = &column_c[0..1];
            let b = &column_c[1..2];
            get_index(a) * 26
                + get_index(b)
        }
        1 => get_index(&column_c[0..1]),
        _ => {
            panic!("longer than 3 characters");
        }
    }
}

fn get_index(column: &str) -> u32 {
    let mut i = 0;
    for tar in self::ALPHABET {
        if tar == &column {
            return i + 1;
        }
        i += 1;
    }
    panic!("illegal character");
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
        static ref RE: Regex = Regex::new(r"[A-Z]+").unwrap();
    }
    let caps = RE.captures(coordinate);
    let col = match caps {
        Some(v) => Some(v.get(0).unwrap().as_str()),
        None => None,
    };
    let is_lock_col = match col {
        Some(v) => match coordinate.find(format!("{}{}", "$", v).as_str()) {
            Some(_) => Some("1"),
            None => Some("0"),
        },
        None => None,
    };

    lazy_static! {
        static ref RE_NUM: Regex = Regex::new(r"[0-9]+").unwrap();
    }
    let caps = RE_NUM.captures(coordinate);
    let row = match caps {
        Some(v) => Some(v.get(0).unwrap().as_str()),
        None => None,
    };
    let is_lock_row = match row {
        Some(v) => match coordinate.find(format!("{}{}", "$", v).as_str()) {
            Some(_) => Some("1"),
            None => Some("0"),
        },
        None => None,
    };

    vec![col, row, is_lock_col, is_lock_row]
}

pub fn coordinate_from_index(col: &u32, row: &u32) -> String {
    format!("{}{}", string_from_column_index(&col), row)
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
        string_from_column_index(&col),
        if is_lock_row == &true { "$" } else { "" },
        row
    )
}

pub fn index_from_coordinate<S: AsRef<str>>(coordinate: S) -> Vec<Option<u32>> {
    let split = coordinate_from_string(coordinate.as_ref());
    let col = match split[0] {
        Some(v) => Some(column_index_from_string(v)),
        None => None,
    };
    let row = match split[1] {
        Some(v) => Some(v.parse::<u32>().unwrap()),
        None => None,
    };
    let is_lock_col = match split[2] {
        Some(v) => Some(v.parse::<u32>().unwrap()),
        None => None,
    };
    let is_lock_row = match split[3] {
        Some(v) => Some(v.parse::<u32>().unwrap()),
        None => None,
    };
    vec![col, row, is_lock_col, is_lock_row]
}

pub(crate) fn adjustment_insert_coordinate(num: &u32, root_num: &u32, offset_num: &u32) -> u32 {
    let mut result = num.clone();
    if (num >= root_num && offset_num > &0) || (num < root_num && offset_num < &0) {
        result += offset_num;
    }
    result
}

pub(crate) fn adjustment_remove_coordinate(num: &u32, root_num: &u32, offset_num: &u32) -> u32 {
    let mut result = num.clone();
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
