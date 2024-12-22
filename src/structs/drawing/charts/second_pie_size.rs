// c:secondPieSize
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::UInt16Value;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct SecondPieSize {
    val: UInt16Value,
}
impl SecondPieSize {
    #[must_use]
    pub fn get_val(&self) -> u16 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: u16) -> &mut SecondPieSize {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:secondPieSize
        write_start_tag(
            writer,
            "c:secondPieSize",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
