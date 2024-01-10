use super::Address;
use super::StringValue;
use helper::address::*;

#[derive(Clone, Default, Debug)]
pub struct DefinedName {
    name: String,
    address: Vec<Address>,
    string_value: StringValue,
    is_local_only: bool,
}
impl DefinedName {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name = value.into();
        self
    }

    pub fn get_address(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.get_value_string().to_string();
        }
        let mut result: Vec<String> = Vec::new();
        for row in &self.address {
            result.push(row.get_address());
        }
        result.join(",")
    }

    pub fn set_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let list = self.split_str(value);
        for v in &list {
            if is_address(&v) {
                self.add_address(v);
            } else {
                self.set_string_value(v);
            }
        }
        self
    }

    pub fn add_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let mut obj = Address::default();
        obj.set_address(value);
        self.address.push(obj);
        self
    }

    pub(crate) fn get_address_obj(&self) -> &Vec<Address> {
        &self.address
    }

    pub(crate) fn get_address_obj_mut(&mut self) -> &mut Vec<Address> {
        &mut self.address
    }

    pub(crate) fn set_string_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.address.clear();
        self.string_value.set_value(value);
        self
    }

    pub fn get_is_local_only(&self) -> &bool {
        &self.is_local_only
    }

    pub fn set_is_local_only(&mut self, value: bool) {
        self.is_local_only = value;
    }

    fn split_str<S: Into<String>>(&self, value: S) -> Vec<String> {
        let value = value.into();
        let char_list: Vec<char> = value.chars().collect::<Vec<char>>();
        let mut is_pass_s = false;
        let mut is_pass_d = false;
        let mut result: Vec<String> = Vec::new();
        let mut string = String::from("");
        for c in &char_list {
            match c {
                '\'' => is_pass_s = !is_pass_s,
                '"' => is_pass_d = !is_pass_d,
                ',' => {
                    if is_pass_s || is_pass_d {
                        result.push(string);
                        string = String::from("");
                    } else {
                        string.push(*c);
                    }
                }
                _ => {
                    string.push(*c);
                }
            }
        }
        if !string.is_empty() {
            result.push(string);
        }
        result
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for address in &mut self.address {
            address.get_range_mut().adjustment_insert_coordinate(
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
        for address in &mut self.address {
            address.get_range_mut().adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    pub(crate) fn is_remove(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.address.retain(|x| {
            !(x.is_remove(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ))
        });
        if self.string_value.has_value() {
            return false;
        }
        if self.address.is_empty() {
            return true;
        }
        false
    }

    pub(crate) fn set_sheet_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        for address in &mut self.address {
            address.set_sheet_name(value.clone());
        }
        self
    }
}
