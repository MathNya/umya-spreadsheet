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
    if column_c == "0" {
        return 0;
    }
    match column_c.len() {
        3 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string()) * 676;
            result += get_index(&column_c.chars().nth(1).unwrap().to_string()) * 26;
            result += get_index(&column_c.chars().nth(2).unwrap().to_string());
            return result;
        },
        2 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string()) * 26;
            result += get_index(&column_c.chars().nth(1).unwrap().to_string());
            return result;
        },
        1 => {
            let mut result = 1;
            result += get_index(&column_c.chars().nth(0).unwrap().to_string());
            return result;
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
            return i;
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
    let mut index_value = column_index.clone();
    while index_value > 0 {
        let character_value = index_value % 26;
        index_value = (index_value - character_value) / 26;
        result = format!("{}{}", self::ALPHABET.get(character_value - 1).unwrap(), result);
    }
    result
}

pub fn coordinate_from_string(coordinate:&str)->Vec<Option<&str>> {
    let re = Regex::new(r"[A-Z]+").unwrap();
    let caps = re.captures(coordinate);
    let col = match caps {Some(v) => Some(v.get(0).unwrap().as_str()), None => None};
    let is_lock_col = match col {
        Some(v) => match coordinate.find(format!("{}{}", "$", v).as_str()) {Some(_) => Some("1"), None => Some("0")},
        None => None,
    };

    let re = Regex::new(r"[0-9]+").unwrap();
    let caps = re.captures(coordinate);
    let row = match caps {Some(v) => Some(v.get(0).unwrap().as_str()), None => None};
    let is_lock_row = match row {
        Some(v) => match coordinate.find(format!("{}{}", "$", v).as_str()) {Some(_) => Some("1"), None => Some("0")},
        None => None,
    };

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

pub fn index_from_coordinate<S: Into<String>>(coordinate:S)->Vec<Option<usize>> {
    let con = coordinate.into();
    let split = coordinate_from_string(con.as_str());
    let col = match split[0] {Some(v) => Some(column_index_from_string(v)), None => None};
    let row = match split[1] {Some(v) => Some(v.parse::<usize>().unwrap()), None => None};
    let is_lock_col = match split[2] {Some(v) => Some(v.parse::<usize>().unwrap()), None => None};
    let is_lock_row = match split[3] {Some(v) => Some(v.parse::<usize>().unwrap()), None => None};
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
