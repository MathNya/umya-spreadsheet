// c:serLines
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct SeriesLines {}
impl SeriesLines {
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _e: &BytesStart) {}

    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:serLines
        write_start_tag(writer, "c:serLines", vec![], true);
    }
}
