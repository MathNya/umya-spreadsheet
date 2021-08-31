use regex::Regex;

const ALPHABET: &'static [&'static str] = &[
    "A", "B", "C", "D", "E",
    "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O",
    "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y",
    "Z",
];

pub fn column_index_from_string<S: Into<String>>(column:S)->usize {
    let column_c = column.into().clone();
    match column_c.len() {
        3 => {
            get_index(&column_c.chars().nth(0).unwrap().to_string()) * 676 +
            get_index(&column_c.chars().nth(1).unwrap().to_string()) * 26 +
            get_index(&column_c.chars().nth(2).unwrap().to_string())
        },
        2 => {
            get_index(&column_c.chars().nth(0).unwrap().to_string()) * 26 +
            get_index(&column_c.chars().nth(1).unwrap().to_string())
        },
        1 => {
            get_index(&column_c.chars().nth(0).unwrap().to_string())
        },
        _ => {
            panic!("longer than 3 characters");
        }
    }
}

fn get_index(column:&str)->usize {
    let mut i = 0;
    for tar in self::ALPHABET {
        if tar == &column {
            return i + 1;
        }
        i += 1;
    }
    panic!("illegal character");
}

pub fn string_from_column_index(column_index:&usize)->String {
    if column_index < &1usize {
        panic!("Column number starts from 1.");
    }
    let mut result: String = String::from("");
    let mut index_value = *column_index;
    while index_value > 0 {
        let character_value = (index_value - 1) % 26 + 1;
        index_value = (index_value - character_value) / 26;
        result = format!("{}{}", self::ALPHABET.get(character_value - 1).unwrap(), result);
    }
    result
}

pub fn coordinate_from_string(coordinate:&str)->Vec<&str> {
    let re = Regex::new(r"[A-Z]+").unwrap();
    let caps = re.captures(coordinate).unwrap();
    let col = caps.get(0).unwrap().as_str();
    let is_lock_col = match coordinate.find(format!("{}{}", "$", col).as_str()) {Some(_) => "1", None => "0"};

    let re = Regex::new(r"[0-9]+").unwrap();
    let caps = re.captures(coordinate).unwrap();
    let row = caps.get(0).unwrap().as_str();
    let is_lock_row = match coordinate.find(format!("{}{}", "$", row).as_str()) {Some(_) => "1", None => "0"};

    vec![col, row, is_lock_col, is_lock_row]
}

pub fn coordinate_from_index(col:&usize, row:&usize)->String {
    format!(
        "{}{}",
        string_from_column_index(&col),
        row
    )
}

pub fn coordinate_from_index_with_lock(col:&usize, row:&usize, is_lock_col:&bool, is_lock_row:&bool)->String {
    format!(
        "{}{}{}{}",
        if is_lock_col == &true {"$"} else {""},
        string_from_column_index(&col),
        if is_lock_row == &true {"$"} else {""},
        row
    )
}

pub fn index_from_coordinate<S: Into<String>>(coordinate:S)->Vec<usize> {
    let con = coordinate.into();
    let split = coordinate_from_string(con.as_str());
    let col = column_index_from_string(split[0]);
    let row = split[1].parse::<usize>().unwrap();
    let is_lock_col = split[2].parse::<usize>().unwrap();
    let is_lock_row = split[3].parse::<usize>().unwrap();
    vec![col, row, is_lock_col, is_lock_row]
}

pub(crate) fn adjustment_insert_coordinate(num:&usize, root_num:&usize, offset_num:&usize)->usize {
    let mut result = num.clone();
    if (num >= root_num && offset_num > &0) || (num < root_num && offset_num < &0) {
        result += offset_num;
    }
    result
}

pub(crate) fn adjustment_remove_coordinate(num:&usize, root_num:&usize, offset_num:&usize)->usize {
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
        assert_eq!(index_from_coordinate("$A$4"), vec![1, 4, 1, 1]);
        assert_eq!(index_from_coordinate("$A4"), vec![1, 4, 1, 0]);
        assert_eq!(index_from_coordinate("A4"), vec![1, 4, 0, 0]);
        assert_eq!(index_from_coordinate("Z91"), vec![26, 91, 0, 0]);
        assert_eq!(index_from_coordinate("AA91"), vec![27, 91, 0, 0]);
        assert_eq!(index_from_coordinate("AA$91"), vec![27, 91, 0, 1]);
        assert_eq!(index_from_coordinate("$AA91"), vec![27, 91, 1, 0]);
        assert_eq!(index_from_coordinate("$AA$91"), vec![27, 91, 1, 1]);
    }
}