// strike
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::BooleanValue;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Strike {
    pub(crate) val: BooleanValue,
}

impl Strike {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> bool {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: bool) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value(true);
        set_string_from_xml!(self, e, val, "val");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // strike
        if !self.val.has_value() {
            return;
        }

        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if !self.val.get_value() {
            attributes.push(("val", self.val.get_value_string()));
        }
        write_start_tag(writer, "strike", attributes, true);
    }
}
