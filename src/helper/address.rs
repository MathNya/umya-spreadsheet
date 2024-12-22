use std::sync::OnceLock;

use fancy_regex::Regex;

static ADDRESS_REGEX: OnceLock<Regex> = OnceLock::new();

#[must_use]
pub fn split_address(address: &str) -> (&str, &str) {
    address.rsplit_once('!').map_or(("", address), |(sheet_name, range)| {
        (sheet_name.trim_matches(&['\'', '"'][..]), range)
    })
}

#[must_use]
pub fn join_address(sheet_name: &str, address: &str) -> String {
    if sheet_name.is_empty() {
        return address.to_string();
    }
    format!("{sheet_name}!{address}")
}

/// Checks if the given input string is a valid address format.
///
/// The address format is defined by the following regular expression:
/// `^([^\:\\\?$$$$\/\*]+\!)?(\$?[A-Z]{1,3}\$?[0-9]+)(\:\$?[A-Z]{1,3}\$?[0-9]+)?
/// $`.
///
/// # Parameters
///
/// - `input`: A string slice that can be converted to a string reference. This
///   is the input string to be checked against the address format.
///
/// # Returns
///
/// Returns `true` if the input string matches the address format, and `false`
/// otherwise.
///
/// # Panics
///
/// This function may panic if the regular expression fails to compile. However,
/// since the regular expression is initialized only once and is hardcoded, this
/// is unlikely to occur unless there is a bug in the regex itself. The panic
/// will occur during the first call to this function if the regex is invalid.
///
/// # Examples
///
/// ```
/// let valid_address = "$A1";
/// assert!(is_address(valid_address));
///
/// let invalid_address = "invalid_address";
/// assert!(!is_address(invalid_address));
/// ```
pub fn is_address<S: AsRef<str>>(input: S) -> bool {
    let regex = ADDRESS_REGEX.get_or_init(|| {
        Regex::new(r"^([^\:\\\?\[\]\/\*]+\!)?(\$?[A-Z]{1,3}\$?[0-9]+)(\:\$?[A-Z]{1,3}\$?[0-9]+)?$")
            .unwrap()
    });
    regex.is_match(input.as_ref()).unwrap()
}

#[test]
fn split_address_test() {
    assert_eq!(split_address("A1"), ("", "A1"));
    assert_eq!(split_address("A1:B2"), ("", "A1:B2"));
    assert_eq!(split_address("sheet1!A1:B2"), ("sheet1", "A1:B2"));
    assert_eq!(split_address("'she!et1'!A1:B2"), ("she!et1", "A1:B2"));
    assert_eq!(split_address(r#"'she"et1'!A1:B2"#), (r#"she"et1"#, "A1:B2"));
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
