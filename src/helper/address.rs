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
