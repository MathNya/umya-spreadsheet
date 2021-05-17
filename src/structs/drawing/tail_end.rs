// a:tailEnd
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct TailEnd {
    r#type: String,
}
impl TailEnd {
    pub fn get_type(&self) -> &str {
        &self.r#type
    }

    pub fn set_type<S: Into<String>>(&mut self, value:S) {
        self.r#type = value.into();
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"type") {
            Some(v) => {&mut self.set_type(v);},
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:tailEnd
        write_start_tag(writer, "a:tailEnd", vec![
            ("type", &self.r#type),
        ], true);
    }
}
