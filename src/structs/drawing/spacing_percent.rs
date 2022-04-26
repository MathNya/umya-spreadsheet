// a:spcPct
use super::super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SpacingPercent {
    val: Int32Value,
}
impl SpacingPercent {
    pub fn get_val(&self) -> &i32 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: i32) -> &mut Self {
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
        // a:spcPct
        write_start_tag(
            writer,
            "a:spcPct",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
