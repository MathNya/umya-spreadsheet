// a:ext
// a:chExt
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::structs::Int64Value;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PositiveSize2DType {
    cx: Int64Value,
    cy: Int64Value,
}

impl PositiveSize2DType {
    #[inline]
    pub fn get_cx(&self) -> &i64 {
        &self.cx.get_value()
    }

    #[inline]
    pub fn set_cx(&mut self, value: i64) {
        self.cx.set_value(value);
    }

    #[inline]
    pub fn get_cy(&self) -> &i64 {
        &self.cy.get_value()
    }

    #[inline]
    pub fn set_cy(&mut self, value: i64) {
        self.cy.set_value(value);
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
    pub(crate) fn write_to_ext(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:ext");
    }

    #[inline]
    pub(crate) fn write_to_ch_ext(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:chExt");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let cx_str = self.cx.get_value_string();
        attributes.push(("cx", &cx_str));
        let cy_str = self.cy.get_value_string();
        attributes.push(("cy", &cy_str));
        write_start_tag(writer, tag_name, attributes, true);
    }
}
