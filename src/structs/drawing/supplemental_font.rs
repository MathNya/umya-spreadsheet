// a:font
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::StringValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct SupplementalFont {
    script:   StringValue,
    typeface: StringValue,
}

impl SupplementalFont {
    #[inline]
    #[must_use]
    pub fn script(&self) -> &str {
        self.script.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use script()")]
    pub fn get_script(&self) -> &str {
        self.script()
    }

    #[inline]
    pub fn set_script<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.script.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn typeface(&self) -> &str {
        self.typeface.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use typeface()")]
    pub fn get_typeface(&self) -> &str {
        self.typeface()
    }

    #[inline]
    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.typeface.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, script, "script");
        set_string_from_xml!(self, e, typeface, "typeface");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:font" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:font")
        );
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
