// family
use super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontCharSet {
    pub(crate) val: Int32Value,
}

impl FontCharSet {
    #[inline]
    pub fn get_val(&self) -> &i32 {
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
        // charset
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let val = self.val.get_value_string();
            attributes.push(("val", &val));
            write_start_tag(writer, "charset", attributes, true);
        }
    }
}
