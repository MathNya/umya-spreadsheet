use fancy_regex::Regex;

pub fn split_address(address: &str) -> (String, String) {
    let mut sheet_name = String::from("");
    let split_value: Vec<&str> = address.split('!').collect();
    let range = if split_value.len() == 1 {
        split_value[0].to_string()
    } else if split_value.len() == 2 {
        sheet_name = split_value[0].to_string();
        split_value[1].to_string()
    } else {
        panic!("Non-standard address");
    };
    (sheet_name, range)
}

pub fn is_address<S: AsRef<str>>(input: S) -> bool {
    let re =
        Regex::new(r"^([^\:\\\?\[\]\/\*]+\!)?(\$?[A-Z]{1,3}\$?[0-9]+)(\:\$?[A-Z]{1,3}\$?[0-9]+)?$")
            .unwrap();
    re.is_match(input.as_ref()).unwrap()
}

#[test]
fn is_address_test() {
    assert_eq!(is_address("A1"), true);
    assert_eq!(is_address("$A1"), true);
    assert_eq!(is_address("A$1"), true);
    assert_eq!(is_address("$A$1"), true);

    assert_eq!(is_address("A1:B2"), true);
    assert_eq!(is_address("$A1:B2"), true);
    assert_eq!(is_address("$A$1:B2"), true);
    assert_eq!(is_address("$A$1:$B2"), true);
    assert_eq!(is_address("$A$1:$B$2"), true);

    assert_eq!(is_address("Sheet1!A1"), true);
    assert_eq!(is_address("Sheet1!$A1"), true);
    assert_eq!(is_address("Sheet1!A$1"), true);
    assert_eq!(is_address("Sheet1!A$1"), true);

    assert_eq!(is_address("Sheet1!A1:B2"), true);
    assert_eq!(is_address("Sheet1!$A1:B2"), true);
    assert_eq!(is_address("Sheet1!$A$1:B2"), true);
    assert_eq!(is_address("Sheet1!$A$1:$B2"), true);
    assert_eq!(is_address("Sheet1!$A$1:$B$2"), true);
    assert_eq!(is_address("New Sheet!$H$7:$H$10"), true);

    assert_eq!(is_address("(Sheet1!A1:B2)"), false);
    assert_eq!(is_address("Sheet1!A1:"), false);
    assert_eq!(is_address("Sheet1!A1:B"), false);
    assert_eq!(is_address("Sheet1!A:B2"), false);
    assert_eq!(is_address("Sheet1"), false);
}
