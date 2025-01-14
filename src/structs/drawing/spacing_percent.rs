// a:spcPct
use std::io::Cursor;

use quick_xml::{events::BytesStart, Reader, Writer};

use super::super::super::Int32Value;
use crate::{reader::driver::get_attribute, writer::driver::write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct SpacingPercent {
    val: Int32Value,
}
impl SpacingPercent {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> i32 {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: i32) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:spcPct
        write_start_tag(
            writer,
            "a:spcPct",
            vec![("val", &self.val.get_value_string()).into()],
            true,
        );
    }
}
