//p:cNvPr
use super::super::super::StringValue;
use super::super::super::UInt32Value;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NonVisualDrawingProperties  {
    name: StringValue,
    id: UInt32Value,
}
impl NonVisualDrawingProperties  {
    pub fn get_name(&self) -> &str {
        &self.name.get_value()
    }

    pub fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name.set_value(value);
    }

    pub fn get_id(&self) -> &u32 {
        &self.id.get_value()
    }

    pub fn set_id(&mut self, value:u32) {
        self.id.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        &mut self.id.set_value_string(get_attribute(e, b"id").unwrap());
        &mut self.name.set_value_string(get_attribute(e, b"name").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xdr:cNvPr", vec![
            ("id", &self.id.get_value_string()),
            ("name", &self.name.get_value_string()),
        ], true);
    }
}
