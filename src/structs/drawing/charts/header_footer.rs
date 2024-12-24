// c:headerFooter
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct HeaderFooter {}
impl HeaderFooter {
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _e: &BytesStart) {}

    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:headerFooter
        write_start_tag(writer, "c:headerFooter", vec![], true);
    }
}
