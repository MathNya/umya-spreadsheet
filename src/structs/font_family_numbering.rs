// family
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::Int32Value;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontFamilyNumbering {
    pub(crate) val: Int32Value,
}

impl FontFamilyNumbering {
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
        set_string_from_xml!(self, e, val, "val");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // family
        if self.val.has_value() {
            write_start_tag(
                writer,
                "family",
                vec![("val", &self.val.get_value_string())],
                true,
            );
        }
    }
}
