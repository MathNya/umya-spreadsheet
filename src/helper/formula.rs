use regex::Regex;

pub fn update(formula:&str, col_base:&usize, col_add:&usize, row_base:&usize, row_num:&usize, worksheet_name:&str)-> String {
    let re = Regex::new(r"\$[A-Z]{1,3}\$\d+").unwrap();
    for caps in re.captures_iter(formula) {
        dbg!("{}", &caps[0]);
    }
    String::from(formula)
}
