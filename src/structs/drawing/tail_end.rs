// a:tailEnd
use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct TailEnd {
    t_type: StringValue,
    width: StringValue,
    length: StringValue,
}

impl TailEnd {
    #[inline]
    pub fn get_type(&self) -> &str {
        self.t_type.get_value_str()
    }

    #[inline]
    pub fn set_type<S: Into<String>>(&mut self, value: S) {
        self.t_type.set_value(value.into());
    }

    #[inline]
    pub fn get_width(&self) -> &str {
        self.width.get_value_str()
    }

    #[inline]
    pub fn set_width<S: Into<String>>(&mut self, value: S) {
        self.width.set_value(value.into());
    }

    #[inline]
    pub fn get_length(&self) -> &str {
        self.length.get_value_str()
    }

    #[inline]
    pub fn set_length<S: Into<String>>(&mut self, value: S) {
        self.length.set_value(value.into());
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        if let Some(v) = get_attribute(e, b"type") {
            self.set_type(v);
        }

        if let Some(v) = get_attribute(e, b"w") {
            self.set_width(v);
        }

        if let Some(v) = get_attribute(e, b"len") {
            self.set_length(v);
        }

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:tailEnd" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:tailEnd")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:tailEnd
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.t_type.has_value() {
            attributes.push(("type", (self.t_type.get_value_str())));
        }
        if self.width.has_value() {
            attributes.push(("w", (self.width.get_value_str())));
        }
        if self.length.has_value() {
            attributes.push(("len", (self.length.get_value_str())));
        }
        write_start_tag(writer, "a:tailEnd", attributes, true);
    }
}
