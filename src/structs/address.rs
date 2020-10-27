use super::range::Range;

#[derive(Clone, Default, Debug)]
pub struct Address {
    sheet_name: String,
    range: Range,
}
impl Address {
    pub(crate) fn set_address<S: Into<String>>(&mut self, value:S) {
        let org_value = value.into().clone();
        let split_value: Vec<&str> = org_value.split("!").collect();

        if split_value.len() == 1 {
            self.range.set_range(split_value[0]);

        } else if split_value.len() == 2 {
            self.sheet_name = split_value[0].to_string();
            self.range.set_range(split_value[1]);

        } else {
            panic!("Non-standard address");
        }
    }

    pub(crate) fn get_address(&self) -> String {
        let range = self.range.get_range();
        if self.sheet_name == "" {
            return range;
        }
        format!("{}!{}", &self.sheet_name, self.range.get_range())
    }
}