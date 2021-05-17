//p:cNvPr
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NonVisualDrawingProperties  {
    name: String,
    id: String,
}
impl NonVisualDrawingProperties  {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_id<S: Into<String>>(&mut self, value:S) {
        self.id = value.into();
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_id(get_attribute(e, b"id").unwrap());
        &mut self.set_name(get_attribute(e, b"name").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xdr:cNvPr", vec![
            ("id", &self.id),
            ("name", &self.name),
        ], true);
    }
}
