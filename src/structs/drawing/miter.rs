// a:miter
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
    Int32Value,
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Miter {
    limit: Int32Value,
}

impl Miter {
    #[inline]
    #[must_use]
    pub fn limit(&self) -> i32 {
        self.limit.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use limit()")]
    pub fn get_limit(&self) -> i32 {
        self.limit()
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
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let lim = self.limit.value_string();
        if self.limit.has_value() {
            attributes.push(("lim", &lim).into());
        }
        write_start_tag(writer, "a:miter", attributes, true);
    }
}
