// c:style
use super::super::super::ByteValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug)]
pub struct Style {
    val: ByteValue,
}
impl Style {
    pub fn get_val(&self) -> &u8 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: u8) -> &mut Style {
        self.val.set_value(value);
        self
    }

    pub(crate) fn _set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn _write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:style
        write_start_tag(
            writer,
            "c:style",
            vec![("val", &self.val.get_value_string())],
            true,
        );
    }
}
