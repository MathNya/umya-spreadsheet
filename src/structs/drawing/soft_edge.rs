// a:softEdge
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::Int64Value;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct SoftEdge {
    radius: Int64Value,
}
impl SoftEdge {
    #[inline]
    #[must_use]
    pub fn get_radius(&self) -> i64 {
        self.radius.get_value()
    }

    #[inline]
    pub fn set_radius(&mut self, value: i64) -> &mut SoftEdge {
        self.radius.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.radius
            .set_value_string(get_attribute(e, b"rad").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:softEdge
        write_start_tag(
            writer,
            "a:softEdge",
            vec![("rad", &self.radius.get_value_string()).into()],
            true,
        );
    }
}
