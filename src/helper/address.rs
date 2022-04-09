pub fn split_address(address: &str) -> (String, String) {
    let mut sheet_name = String::from("");
    let mut range = String::from("");
    let split_value: Vec<&str> = value.split('!').collect();
    if split_value.len() == 1 {
        range = split_value[0].to_string();
    } else if split_value.len() == 2 {
        sheet_name = split_value[0].to_string();
        range = split_value[1].to_string();
    } else {
        panic!("Non-standard address");
    }
    (sheet_name, range)
}
