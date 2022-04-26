// a:ext
use super::super::super::Int64Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Extents {
    cx: Int64Value,
    cy: Int64Value,
}
impl Extents {
    pub fn get_cx(&self) -> &i64 {
        self.cx.get_value()
    }

    pub fn set_cx(&mut self, value: i64) -> &mut Extents {
        self.cx.set_value(value);
        self
    }

    pub fn get_cy(&self) -> &i64 {
        self.cy.get_value()
    }

    pub fn set_cy(&mut self, value: i64) -> &mut Extents {
        self.cy.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.cx.set_value_string(get_attribute(e, b"cx").unwrap());
        self.cy.set_value_string(get_attribute(e, b"cy").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ext
        write_start_tag(
            writer,
            "a:ext",
            vec![
                ("cx", &self.cx.get_value_string()),
                ("cy", &self.cy.get_value_string()),
            ],
            true,
        );
    }
}
