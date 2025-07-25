// a:miter
use crate::reader::driver::*;
use crate::writer::driver::*;
use crate::Int32Value;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Miter {
    limit: Int32Value,
}

impl Miter {
    #[inline]
    pub fn get_limit(&self) -> &i32 {
        self.limit.get_value()
    }

    #[inline]
    pub fn set_limit(&mut self, value: i32) -> &mut Self {
        self.limit.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, limit, "lim");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:miter" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:miter")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:miter
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let lim = self.limit.get_value_string();
        if self.limit.has_value() {
            attributes.push(("lim", &lim));
        }
        write_start_tag(writer, "a:miter", attributes, true);
    }
}
