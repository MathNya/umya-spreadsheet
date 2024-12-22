// a:round
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesStart;

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct Round {}
impl Round {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:round
        write_start_tag(writer, "a:round", vec![], true);
    }
}
