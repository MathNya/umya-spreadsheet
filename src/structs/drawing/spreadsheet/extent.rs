// xdr:ext
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::Int64Value;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Extent {
    cx: Int64Value,
    cy: Int64Value,
}

impl Extent {
    #[inline]
    #[must_use]
    pub fn cx(&self) -> i64 {
        self.cx.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cx()")]
    pub fn get_cx(&self) -> i64 {
        self.cx()
    }

    #[inline]
    pub fn set_cx(&mut self, value: i64) -> &mut Extent {
        self.cx.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn cy(&self) -> i64 {
        self.cy.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cy()")]
    pub fn get_cy(&self) -> i64 {
        self.cy()
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
                ("cx", self.cx.value_string()).into(),
                ("cy", self.cy.value_string()).into(),
            ],
            true,
        );
    }
}
