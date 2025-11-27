// sz
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::DoubleValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct FontSize {
    pub(crate) val: DoubleValue,
}

impl FontSize {
    #[inline]
    #[must_use]
    pub fn val(&self) -> f64 {
        self.val.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> f64 {
        self.val()
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
                vec![("val", &self.val.value_string()).into()],
                true,
            );
        }
    }
}
