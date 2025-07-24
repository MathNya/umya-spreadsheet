// a:font
use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SupplementalFont {
    script: StringValue,
    typeface: StringValue,
}

impl SupplementalFont {
    #[inline]
    pub fn get_script(&self) -> &str {
        self.script.get_value_str()
    }

    #[inline]
    pub fn set_script<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.script.set_value(value);
        self
    }

    #[inline]
    pub fn get_typeface(&self) -> &str {
        self.typeface.get_value_str()
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
                ("script", self.script.get_value_str()),
                ("typeface", self.typeface.get_value_str()),
            ],
            true,
        );
    }
}
