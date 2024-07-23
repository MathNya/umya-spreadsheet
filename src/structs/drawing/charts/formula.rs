use crate::xml_read_loop;

// c:f
use super::super::super::Address;
use super::super::super::StringValue;
use helper::address::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use traits::AdjustmentCoordinateWithSheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Formula {
    address: Address,
    string_value: StringValue,
}

impl Formula {
    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    pub fn get_address_str(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.get_value_str().to_string();
        }
        self.address.get_address()
    }

    pub fn set_address(&mut self, value: Address) -> &mut Self {
        self.address = value;
        self.string_value.remove_value();
        self
    }

    pub fn set_string_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.address = Address::default();
        self.string_value.set_value(value);
        self
    }

    pub fn set_address_str<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        if is_address(&value) {
            self.address.set_address(value);
        } else {
            self.set_string_value(value);
        }
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_address_str(e.unescape().unwrap());
            },
            Event::End(ref e) => {
               if  e.name().0 == b"c:f" {
                   return;
               }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:f"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:f
        write_start_tag(writer, "c:f", vec![], false);
        write_text_node_no_escape(writer, self.get_address_str());
        write_end_tag(writer, "c:f");
    }
}
impl AdjustmentCoordinateWithSheet for Formula {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.address.adjustment_insert_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.address.adjustment_remove_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
