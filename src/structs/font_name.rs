// name
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::StringValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontName {
    pub(crate) val: StringValue,
}

impl FontName {
    #[inline]
    #[must_use]
    pub fn val(&self) -> &str {
        self.val.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> &str {
        self.val()
    }

    #[inline]
    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
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
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // name, rFont
        if self.val.has_value() {
            write_start_tag(
                writer,
                tag_name,
                vec![("val", self.val.value_str()).into()],
                true,
            );
        }
    }
}
