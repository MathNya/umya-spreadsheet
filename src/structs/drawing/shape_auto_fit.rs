// a:spAutoFit
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ShapeAutoFit {
}
impl ShapeAutoFit {

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _:&BytesStart
    ) {
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:spAutoFit
        write_start_tag(writer, "a:spAutoFit", vec![], true);
    }
}
