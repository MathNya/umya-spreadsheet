// c:axId
use super::super::super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct AxisId {
    val: UInt32Value,
}
impl AxisId {
    pub fn get_val(&self) -> &u32 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: u32) -> &mut AxisId {
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
        // c:axId
        write_start_tag(
            writer,
            "c:axId",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
