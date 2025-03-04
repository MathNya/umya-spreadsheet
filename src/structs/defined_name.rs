use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Address,
    BooleanValue,
    StringValue,
    UInt32Value,
};
use crate::{
    helper::address::is_address,
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    traits::AdjustmentCoordinateWithSheet,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node_conversion,
    },
};

#[derive(Clone, Default, Debug)]
pub struct DefinedName {
    name:           StringValue,
    address:        Vec<Address>,
    string_value:   StringValue,
    local_sheet_id: UInt32Value,
    hidden:         BooleanValue,
}
impl DefinedName {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[must_use]
    pub fn address(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.value_str().to_string();
        }
        let mut result: Vec<String> = Vec::with_capacity(self.address.len());
        for row in &self.address {
            result.push(row.address_ptn2());
        }
        result.join(",")
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use address()")]
    pub fn get_address(&self) -> String {
        self.address()
    }

    pub fn set_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let list = Self::split_str(value);
        for v in &list {
            if is_address(v) {
                self.add_address(v);
            } else {
                self.set_string_value(v);
            }
        }
        self
    }

    pub fn add_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let mut obj = Address::default();
        obj.set_address(value.into().replace("''", "'"));
        self.address.push(obj);
        self
    }

    #[inline]
    pub(crate) fn address_obj(&self) -> &[Address] {
        &self.address
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use address_obj()")]
    pub(crate) fn get_address_obj(&self) -> &[Address] {
        self.address_obj()
    }

    #[inline]
    pub(crate) fn set_string_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.address.clear();
        self.string_value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn has_local_sheet_id(&self) -> bool {
        self.local_sheet_id.has_value()
    }

    #[inline]
    #[must_use]
    pub fn local_sheet_id(&self) -> u32 {
        self.local_sheet_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use local_sheet_id()")]
    pub fn get_local_sheet_id(&self) -> u32 {
        self.local_sheet_id()
    }

    #[inline]
    pub fn set_local_sheet_id(&mut self, value: u32) {
        self.local_sheet_id.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn hidden(&self) -> bool {
        self.hidden.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use hidden()")]
    pub fn get_hidden(&self) -> bool {
        self.hidden()
    }

    #[inline]
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden.set_value(value);
    }

    fn split_str<S: Into<String>>(value: S) -> Vec<String> {
        let value = value.into();
        let char_list: Vec<char> = value.chars().collect::<Vec<char>>();
        let mut is_pass_s = false;
        let mut is_pass_d = false;
        let mut is_pass_b = 0;
        let mut result: Vec<String> = Vec::new();
        let mut string = String::new();
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

        let mut value: String = String::new();
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
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("name", self.name()).into());
        let local_sheet_id_str = self.local_sheet_id.value_string();
        if self.local_sheet_id.has_value() {
            attributes.push(("localSheetId", &local_sheet_id_str).into());
        }
        let hidden_str = self.hidden.value_string();
        if self.hidden.has_value() {
            attributes.push(("hidden", hidden_str).into());
        }
        write_start_tag(writer, "definedName", attributes, false);
        write_text_node_conversion(writer, self.address());
        write_end_tag(writer, "definedName");
    }
}
impl AdjustmentCoordinateWithSheet for DefinedName {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
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
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
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

    #[inline]
    #[allow(unused_variables)]
    fn is_remove_coordinate_with_sheet(
        &self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
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
