// a:miter
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::writer::driver::write_start_tag;
use crate::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Miter {
    limit: Int32Value,
}

impl Miter {
    #[inline]
    #[must_use]
    pub fn get_limit(&self) -> i32 {
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
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, limit, "lim");
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
