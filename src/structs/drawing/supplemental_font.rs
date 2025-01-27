// a:font
use std::io::Cursor;

use quick_xml::{Reader, Writer, events::BytesStart};

use crate::{
    reader::driver::{get_attribute, set_string_from_xml},
    structs::StringValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct SupplementalFont {
    script: StringValue,
    typeface: StringValue,
}

impl SupplementalFont {
    #[inline]
    #[must_use]
    pub fn get_script(&self) -> &str {
        self.script.value_str()
    }

    #[inline]
    pub fn set_script<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.script.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_typeface(&self) -> &str {
        self.typeface.value_str()
    }

    #[inline]
    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.typeface.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, script, "script");
        set_string_from_xml!(self, e, typeface, "typeface");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:font
        write_start_tag(
            writer,
            "a:font",
            vec![
                ("script", self.script.value_str()).into(),
                ("typeface", self.typeface.value_str()).into(),
            ],
            true,
        );
    }
}
