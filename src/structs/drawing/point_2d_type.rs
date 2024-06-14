// a:off
// a:chOff
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::Int64Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Point2DType {
    x: Int64Value,
    y: Int64Value,
}

impl Point2DType {
    pub fn get_x(&self) -> &i64 {
        &self.x.get_value()
    }

    pub fn set_x(&mut self, value: i64) {
        self.x.set_value(value);
    }

    pub fn get_y(&self) -> &i64 {
        &self.y.get_value()
    }

    pub fn set_y(&mut self, value: i64) {
        self.y.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, x, "x");
        set_string_from_xml!(self, e, y, "y");
    }

    pub(crate) fn write_to_off(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:off");
    }

    pub(crate) fn write_to_ch_off(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:chOff");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let x_str = self.x.get_value_string();
        attributes.push(("x", &x_str));
        let y_str = self.y.get_value_string();
        attributes.push(("y", &y_str));
        write_start_tag(writer, tag_name, attributes, true);
    }
}
