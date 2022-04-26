// a:stCxn
use super::super::super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct StartConnection {
    id: UInt32Value,
    index: UInt32Value,
}
impl StartConnection {
    pub fn get_id(&self) -> &u32 {
        self.id.get_value()
    }

    pub fn set_id(&mut self, value: u32) {
        self.id.set_value(value);
    }

    pub fn get_index(&self) -> &u32 {
        self.index.get_value()
    }

    pub fn set_index(&mut self, value: u32) {
        self.index.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.id.set_value_string(get_attribute(e, b"id").unwrap());
        self.index
            .set_value_string(get_attribute(e, b"idx").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(
            writer,
            "a:stCxn",
            vec![
                ("id", &self.id.get_value_string()),
                ("idx", &self.index.get_value_string()),
            ],
            true,
        );
    }
}
