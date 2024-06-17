use fancy_regex::Regex;

pub fn split_address(address: &str) -> (&str, &str) {
    address
        .rsplit_once('!')
        .map(|(sheet_name, range)| (sheet_name.trim_matches(&['\'', '"'][..]), range))
        .unwrap_or(("", address))
}

#[test]
fn split_address_test() {
    assert_eq!(split_address("A1"), ("", "A1"));
    assert_eq!(split_address("A1:B2"), ("", "A1:B2"));
    assert_eq!(split_address("sheet1!A1:B2"), ("sheet1", "A1:B2"));
    assert_eq!(split_address("'she!et1'!A1:B2"), ("she!et1", "A1:B2"));
    assert_eq!(split_address(r#"'she"et1'!A1:B2"#), (r#"she"et1"#, "A1:B2"));
}

pub fn is_address<S: AsRef<str>>(input: S) -> bool {
    let re =
        Regex::new(r"^([^\:\\\?\[\]\/\*]+\!)?(\$?[A-Z]{1,3}\$?[0-9]+)(\:\$?[A-Z]{1,3}\$?[0-9]+)?$")
            .unwrap();
    re.is_match(input.as_ref()).unwrap()
}

#[test]
fn is_address_test() {
    assert!(is_address("A1"));
    assert!(is_address("$A1"));
    assert!(is_address("A$1"));
    assert!(is_address("$A$1"));

    assert!(is_address("A1:B2"));
    assert!(is_address("$A1:B2"));
    assert!(is_address("$A$1:B2"));
    assert!(is_address("$A$1:$B2"));
    assert!(is_address("$A$1:$B$2"));

    assert!(is_address("Sheet1!A1"));
    assert!(is_address("Sheet1!$A1"));
    assert!(is_address("Sheet1!A$1"));
    assert!(is_address("Sheet1!A$1"));

    assert!(is_address("Sheet1!A1:B2"));
    assert!(is_address("Sheet1!$A1:B2"));
    assert!(is_address("Sheet1!$A$1:B2"));
    assert!(is_address("Sheet1!$A$1:$B2"));
    assert!(is_address("Sheet1!$A$1:$B$2"));
    assert!(is_address("New Sheet!$H$7:$H$10"));

    assert!(!is_address("(Sheet1!A1:B2)"));
    assert!(!is_address("Sheet1!A1:"));
    assert!(!is_address("Sheet1!A1:B"));
    assert!(!is_address("Sheet1!A:B2"));
    assert!(!is_address("Sheet1"));
}
