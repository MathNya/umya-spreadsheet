use super::Range;

#[derive(Clone, Default, Debug)]
pub struct Address {
    sheet_name: String,
    range: Range,
}
impl Address {
    pub fn get_sheet_name(&self) -> &str {
        &self.sheet_name
    }

    pub fn set_sheet_name<S: Into<String>>(&mut self, value: S) -> &mut Address {
        self.sheet_name = value.into();
        self
    }

    pub fn get_range(&self) -> &Range {
        &self.range
    }

    pub fn get_range_mut(&mut self) -> &mut Range {
        &mut self.range
    }

    pub fn set_range(&mut self, value: Range) -> &mut Address {
        self.range = value;
        self
    }

    pub fn set_address<S: Into<String>>(&mut self, value: S) -> &mut Address {
        let org_value = value.into();
        let split_value: Vec<&str> = org_value.split('!').collect();

        if split_value.len() == 1 {
            self.range.set_range(split_value[0]);
        } else if split_value.len() == 2 {
            self.sheet_name = split_value[0].to_string();
            self.range.set_range(split_value[1]);
        } else {
            panic!("Non-standard address");
        }
        self
    }

    pub fn get_address(&self) -> String {
        let range = self.range.get_range();
        if self.sheet_name.is_empty() {
            return range;
        }
        let mut with_space_char = String::from("");
        match self.get_sheet_name().find(char::is_whitespace) {
            Some(_) => {
                with_space_char = String::from("'");
            }
            None => {}
        }
        format!(
            "{}{}{}!{}",
            &with_space_char,
            &self.sheet_name,
            &with_space_char,
            self.range.get_range()
        )
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if &self.sheet_name == sheet_name {
            self.range.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if &self.sheet_name == sheet_name {
            self.range.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    pub(crate) fn is_remove(
        &self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        if &self.sheet_name == sheet_name {
            return self.range.is_remove(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        false
    }
}
