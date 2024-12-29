// scheme
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    EnumValue,
    FontSchemeValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontScheme {
    pub(crate) val: EnumValue<FontSchemeValues>,
}

impl FontScheme {
    #[must_use]
    pub fn get_val(&self) -> &FontSchemeValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: FontSchemeValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // scheme
        if self.val.has_value() {
            let attributes = vec![("val", self.val.get_value_string()).into()];
            write_start_tag(writer, "scheme", attributes, true);
        }
    }
}
