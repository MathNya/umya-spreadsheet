// a:miter
use super::super::super::Int32Value;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Miter {
    limit: Int32Value,
}
impl Miter {
    pub fn get_limit(&self)-> &i32 {
        &self.limit.get_value()
    }
    
    pub fn set_limit(&mut self, value:i32)-> &mut Self {
        self.limit.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"lim") {
            Some(v) => {self.limit.set_value_string(v);},
            None => {},
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:miter
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if &self.limit.has_value() == &true {
            attributes.push(("lim", &self.limit.get_value_string()));
        }
        write_start_tag(writer, "a:miter", attributes, true);
    }
}
