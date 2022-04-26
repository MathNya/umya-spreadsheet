// a:miter
use super::super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Miter {
    limit: Int32Value,
}
impl Miter {
    pub fn get_limit(&self) -> &i32 {
        self.limit.get_value()
    }

    pub fn set_limit(&mut self, value: i32) -> &mut Self {
        self.limit.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"lim") {
            Some(v) => {
                self.limit.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:miter
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let lim = self.limit.get_value_string();
        if &self.limit.has_value() == &true {
            attributes.push(("lim", &lim));
        }
        write_start_tag(writer, "a:miter", attributes, true);
    }
}
