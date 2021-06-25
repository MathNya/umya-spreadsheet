// c:headerFooter
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct HeaderFooter {

}
impl HeaderFooter {

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {

    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:headerFooter
        write_start_tag(writer, "c:headerFooter", vec![], true);
    }
}
