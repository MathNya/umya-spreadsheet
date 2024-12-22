// a:stCxn
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;

use super::super::super::UInt32Value;
use crate::reader::driver::get_attribute;
use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct StartConnection {
    id: UInt32Value,
    index: UInt32Value,
}
impl StartConnection {
    #[inline]
    #[must_use]
    pub fn get_id(&self) -> u32 {
        self.id.get_value()
    }

    #[inline]
    pub fn set_id(&mut self, value: u32) {
        self.id.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_index(&self) -> u32 {
        self.index.get_value()
    }

    #[inline]
    pub fn set_index(&mut self, value: u32) {
        self.index.set_value(value);
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.id.set_value_string(get_attribute(e, b"id").unwrap());
        self.index.set_value_string(get_attribute(e, b"idx").unwrap());
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:stCxn",
            vec![("id", &self.id.get_value_string()), ("idx", &self.index.get_value_string())],
            true,
        );
    }
}
