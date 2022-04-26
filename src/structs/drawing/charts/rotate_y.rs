// c:rotY
use super::super::super::UInt16Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct RotateY {
    val: UInt16Value,
}
impl RotateY {
    pub fn get_val(&self) -> &u16 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: u16) -> &mut RotateY {
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
        // c:rotY
        write_start_tag(
            writer,
            "c:rotY",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
