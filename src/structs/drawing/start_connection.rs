// a:stCxn
use std::io::Cursor;

use quick_xml::{Reader, Writer, events::BytesStart};

use super::super::super::UInt32Value;
use crate::{reader::driver::get_attribute, writer::driver::write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct StartConnection {
    id: UInt32Value,
    index: UInt32Value,
}
impl StartConnection {
    #[inline]
    #[must_use]
    pub fn get_id(&self) -> u32 {
        self.id.value()
    }

    #[inline]
    pub fn set_id(&mut self, value: u32) {
        self.id.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_index(&self) -> u32 {
        self.index.value()
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
        self.index
            .set_value_string(get_attribute(e, b"idx").unwrap());
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:stCxn",
            vec![
                ("id", self.id.value_string()).into(),
                ("idx", self.index.value_string()).into(),
            ],
            true,
        );
    }
}
