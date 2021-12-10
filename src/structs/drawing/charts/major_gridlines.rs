// c:majorGridlines
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct MajorGridlines {
}
impl MajorGridlines {

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:majorGridlines
        write_start_tag(writer, "c:majorGridlines", vec![], true);
    }
}
