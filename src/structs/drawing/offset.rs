// a:off
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::Int64Value;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Offset {
    x: Int64Value,
    y: Int64Value,
}
impl Offset {
    #[inline]
    #[must_use]
    pub fn get_x(&self) -> i64 {
        self.x.get_value()
    }

    #[inline]
    pub fn set_x(&mut self, value: i64) {
        self.x.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_y(&self) -> i64 {
        self.y.get_value()
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
        self.x.set_value_string(get_attribute(e, b"x").unwrap());
        self.y.set_value_string(get_attribute(e, b"y").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:off
        write_start_tag(
            writer,
            "a:off",
            vec![("x", &self.x.get_value_string()), ("y", &self.y.get_value_string())],
            true,
        );
    }
}
