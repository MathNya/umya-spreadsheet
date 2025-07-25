// a:sysClr
use super::super::super::EnumValue;
use super::super::super::StringValue;
use super::SystemColorValues;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SystemColor {
    val: EnumValue<SystemColorValues>,
    last_color: StringValue,
}

impl SystemColor {
    #[inline]
    pub fn get_val(&self) -> &SystemColorValues {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: SystemColorValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub fn get_last_color(&self) -> &str {
        self.last_color.get_value_str()
    }

    #[inline]
    pub fn set_last_color<S: Into<String>>(&mut self, value: S) {
        self.last_color.set_value(value.into());
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, val, "val");
        set_string_from_xml!(self, e, last_color, "lastClr");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:sysClr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:sysClr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srgbClr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let val = self.val.get_value_string();
        if self.val.has_value() {
            attributes.push(("val", val));
        }
        let last_color = self.last_color.get_value_str();
        if self.last_color.has_value() {
            attributes.push(("lastClr", last_color));
        }
        write_start_tag(writer, "a:sysClr", attributes, true);
    }
}
