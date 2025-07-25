use super::super::Int32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PositiveFixedPercentageType {
    val: Int32Value,
}

impl PositiveFixedPercentageType {
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
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, val, "val");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:shade" {
                    return;
                }
                if e.name().into_inner() == b"a:alpha" {
                    return;
                }
                if e.name().into_inner() == b"a:tint" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:shade,a:alpha,a:tint")
        );
    }

    #[inline]
    pub(crate) fn write_to_shade(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:shade")
    }

    #[inline]
    pub(crate) fn write_to_alpha(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:alpha")
    }

    #[inline]
    pub(crate) fn write_to_tint(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:tint")
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tab_name: &str) {
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let val = self.val.get_value_string();
            attributes.push(("val", &val));
            write_start_tag(writer, tab_name, attributes, true);
        }
    }
}
