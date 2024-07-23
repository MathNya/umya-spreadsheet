use super::Address;
use super::BooleanValue;
use super::StringValue;
use super::UInt32Value;
use helper::address::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct DefinedName {
    name: StringValue,
    address: Vec<Address>,
    string_value: StringValue,
    local_sheet_id: UInt32Value,
    hidden: BooleanValue,
}
impl DefinedName {
    pub fn get_name(&self) -> &str {
        &self.name.get_value_str()
    }

    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_address(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.get_value_str().to_string();
        }
        let mut result: Vec<String> = Vec::new();
        for row in &self.address {
            result.push(row.get_address_ptn2());
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
        let mut value = value.into();
        value = value.replace("''", "'");
        let mut obj = Address::default();
        obj.set_address(value);
        self.address.push(obj);
        self
    }

    pub(crate) fn get_sheet_name_crate(&self) -> String {
        if self.string_value.has_value() {
            return String::from("");
        }
        self.address
            .first()
            .unwrap_or(&Address::default())
            .get_sheet_name()
            .to_string()
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

    pub fn has_local_sheet_id(&self) -> bool {
        self.local_sheet_id.has_value()
    }

    pub fn get_local_sheet_id(&self) -> &u32 {
        &self.local_sheet_id.get_value()
    }

    pub fn set_local_sheet_id(&mut self, value: u32) {
        self.local_sheet_id.set_value(value);
    }

    pub fn get_hidden(&self) -> &bool {
        &self.hidden.get_value()
    }

    pub fn set_hidden(&mut self, value: bool) {
        self.hidden.set_value(value);
    }

    fn split_str<S: Into<String>>(&self, value: S) -> Vec<String> {
        let value = value.into();
        let char_list: Vec<char> = value.chars().collect::<Vec<char>>();
        let mut is_pass_s = false;
        let mut is_pass_d = false;
        let mut is_pass_b = 0;
        let mut result: Vec<String> = Vec::new();
        let mut string = String::from("");
        for c in &char_list {
            match c {
                '(' => {
                    is_pass_b += 1;
                    string.push(*c);
                }
                ')' => {
                    is_pass_b -= 1;
                    string.push(*c);
                }
                '\'' => {
                    is_pass_s = !is_pass_s;
                    string.push(*c);
                }
                '"' => {
                    is_pass_d = !is_pass_d;
                    if is_pass_s || is_pass_b != 0 {
                        string.push(*c);
                    }
                }
                ',' => {
                    if !is_pass_s && !is_pass_d && is_pass_b == 0 {
                        result.push(std::mem::take(&mut string));
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

    pub(crate) fn set_sheet_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        for address in &mut self.address {
            address.set_sheet_name(value.clone());
        }
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, local_sheet_id, "localSheetId");
        set_string_from_xml!(self, e, hidden, "hidden");

        let mut value: String = String::from("");
        xml_read_loop!(
            reader,
                Event::Text(e) => {
                    value = e.unescape().unwrap().to_string();
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"definedName" {
                        self.set_address(value.clone());
                        return
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "definedName")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // definedName
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("name", self.get_name()));
        let local_sheet_id_str = self.local_sheet_id.get_value_string();
        if self.local_sheet_id.has_value() {
            attributes.push(("localSheetId", &local_sheet_id_str));
        }
        let hidden_str = self.hidden.get_value_string();
        if self.hidden.has_value() {
            attributes.push(("hidden", &hidden_str));
        }
        write_start_tag(writer, "definedName", attributes, false);
        write_text_node_no_escape(writer, self.get_address());
        write_end_tag(writer, "definedName");
    }
}
impl AdjustmentCoordinateWithSheet for DefinedName {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for address in &mut self.address {
            address.adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.address.retain(|x| {
            !(x.is_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ))
        });
        for address in &mut self.address {
            address.adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn is_remove_coordinate_with_sheet(
        &self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        if self.string_value.has_value() {
            return false;
        }
        if self.address.is_empty() {
            return true;
        }
        false
    }
}
