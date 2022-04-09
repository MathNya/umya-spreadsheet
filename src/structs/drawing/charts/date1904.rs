// c:h
use super::super::super::BooleanValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Date1904 {
    val: BooleanValue,
}
impl Date1904 {
    pub fn get_val(&self) -> &bool {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: bool) -> &mut Date1904 {
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
        // c:date1904
        write_start_tag(
            writer,
            "c:date1904",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
