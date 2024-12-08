// c:thickness
use super::super::super::ByteValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Thickness {
    val: ByteValue,
}
impl Thickness {
    pub fn get_val(&self) -> &u8 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: u8) -> &mut Thickness {
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
        // c:thickness
        write_start_tag(
            writer,
            "c:thickness",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
