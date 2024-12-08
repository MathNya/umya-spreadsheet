// sz
use super::DoubleValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct FontSize {
    pub(crate) val: DoubleValue,
}

impl FontSize {
    #[inline]
    pub fn get_val(&self) -> &f64 {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: f64) -> &mut Self {
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
        // sz
        if self.val.has_value() {
            write_start_tag(
                writer,
                "sz",
                vec![("val", &self.val.get_value_string())],
                true,
            );
        }
    }
}
