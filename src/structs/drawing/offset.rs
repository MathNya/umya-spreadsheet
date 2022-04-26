// a:off
use super::super::super::Int64Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Offset {
    x: Int64Value,
    y: Int64Value,
}
impl Offset {
    pub fn get_x(&self) -> &i64 {
        self.x.get_value()
    }

    pub fn set_x(&mut self, value: i64) {
        self.x.set_value(value);
    }

    pub fn get_y(&self) -> &i64 {
        self.y.get_value()
    }

    pub fn set_y(&mut self, value: i64) {
        self.y.set_value(value);
    }

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
            vec![
                ("x", &self.x.get_value_string()),
                ("y", &self.y.get_value_string()),
            ],
            true,
        );
    }
}
