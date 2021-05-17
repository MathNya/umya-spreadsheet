use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ConnectionType {
    id: String,
    index: String,
}
impl ConnectionType {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_id<S: Into<String>>(&mut self, value:S) {
        self.id = value.into();
    }

    pub fn get_index(&self) -> &str {
        &self.index
    }

    pub fn set_index<S: Into<String>>(&mut self, value:S) {
        self.index = value.into();
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_id(get_attribute(e, b"id").unwrap());
        &mut self.set_index(get_attribute(e, b"idx").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        write_start_tag(writer, tag_name, vec![
            ("id", &self.id),
            ("idx", &self.index),
        ], true);
    }
}