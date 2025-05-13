// a:off
// a:chOff
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::Int64Value,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Point2DType {
    x: Int64Value,
    y: Int64Value,
}

impl Point2DType {
    #[inline]
    #[must_use]
    pub fn x(&self) -> i64 {
        self.x.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use x()")]
    pub fn get_x(&self) -> i64 {
        self.x()
    }

    #[inline]
    pub fn set_x(&mut self, value: i64) {
        self.x.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn y(&self) -> i64 {
        self.y.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use y()")]
    pub fn get_y(&self) -> i64 {
        self.y()
    }

    #[inline]
    pub fn set_y(&mut self, value: i64) {
        self.y.set_value(value);
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, x, "x");
        set_string_from_xml!(self, e, y, "y");
    }

    #[inline]
    pub(crate) fn write_to_off(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:off");
    }

    #[inline]
    pub(crate) fn write_to_ch_off(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:chOff");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let x_str = self.x.value_string();
        attributes.push(("x", &x_str).into());
        let y_str = self.y.value_string();
        attributes.push(("y", &y_str).into());
        write_start_tag(writer, tag_name, attributes, true);
    }
}
