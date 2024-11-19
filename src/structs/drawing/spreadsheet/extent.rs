// xdr:ext
use super::super::super::Int64Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Extent {
    cx: Int64Value,
    cy: Int64Value,
}

impl Extent {
    #[inline]
    pub fn get_cx(&self) -> &i64 {
        self.cx.get_value()
    }

    #[inline]
    pub fn set_cx(&mut self, value: i64) -> &mut Extent {
        self.cx.set_value(value);
        self
    }

    #[inline]
    pub fn get_cy(&self) -> &i64 {
        self.cy.get_value()
    }

    #[inline]
    pub fn set_cy(&mut self, value: i64) -> &mut Extent {
        self.cy.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, cx, "cx");
        set_string_from_xml!(self, e, cy, "cy");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:ext
        write_start_tag(
            writer,
            "xdr:ext",
            vec![
                ("cx", &self.cx.get_value_string()),
                ("cy", &self.cy.get_value_string()),
            ],
            true,
        );
    }
}
