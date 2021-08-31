// xdr:ext
use super::super::super::Int64Value;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Extent {
    cx: Int64Value,
    cy: Int64Value,
}
impl Extent {
    pub fn get_cx(&self) -> &i64 {
        &self.cx.get_value()
    }

    pub fn set_cx(&mut self, value:i64) -> &mut Extent {
        self.cx.set_value(value);
        self
    }

    pub fn get_cy(&self) -> &i64 {
        &self.cy.get_value()
    }

    pub fn set_cy(&mut self, value:i64) -> &mut Extent {
        self.cy.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"cx") {
            Some(v) => {&mut self.cx.set_value_string(v);},
            None => {}
        }
        match get_attribute(e, b"cy") {
            Some(v) => {&mut self.cy.set_value_string(v);},
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:ext
        write_start_tag(writer, "xdr:ext", vec![
            ("cx",  &self.cx.get_value_string()),
            ("cy",  &self.cy.get_value_string()),
        ], true);
    }
}
