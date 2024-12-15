use super::StringValue;
use super::UInt32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Location {
    reference: StringValue,
    first_header_row: UInt32Value,
    first_data_row: UInt32Value,
    first_data_col: UInt32Value,
}
impl Location {
    pub fn get_reference(&self) -> &str {
        self.reference.get_value_str()
    }

    pub fn set_reference<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.reference.set_value(value);
        self
    }

    pub fn get_first_header_row(&self) -> &u32 {
        self.first_header_row.get_value()
    }

    pub fn set_first_header_row(&mut self, value: u32) -> &mut Self {
        self.first_header_row.set_value(value);
        self
    }

    pub fn get_first_data_row(&self) -> &u32 {
        self.first_data_row.get_value()
    }

    pub fn set_first_data_row(&mut self, value: u32) -> &mut Self {
        self.first_data_row.set_value(value);
        self
    }

    pub fn get_first_data_col(&self) -> &u32 {
        self.first_data_col.get_value()
    }

    pub fn set_first_data_col(&mut self, value: u32) -> &mut Self {
        self.first_data_col.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, reference, "ref");
        set_string_from_xml!(self, e, first_header_row, "firstHeaderRow");
        set_string_from_xml!(self, e, first_data_row, "firstDataRow");
        set_string_from_xml!(self, e, first_data_col, "firstDataCol");
    }

    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // location
        write_start_tag(
            writer,
            "location",
            vec![
                ("ref", self.reference.get_value_str()),
                (
                    "firstHeaderRow",
                    self.first_header_row.get_value_string().as_str(),
                ),
                (
                    "firstDataRow",
                    self.first_data_row.get_value_string().as_str(),
                ),
                (
                    "firstDataCol",
                    self.first_data_col.get_value_string().as_str(),
                ),
            ],
            true,
        );
    }
}
