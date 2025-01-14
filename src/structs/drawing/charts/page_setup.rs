// c:pageSetup
use std::io::Cursor;

use quick_xml::{events::BytesStart, Reader, Writer};

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct PageSetup {}
impl PageSetup {
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _e: &BytesStart) {}

    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:pageSetup
        write_start_tag(writer, "c:pageSetup", vec![], true);
    }
}
