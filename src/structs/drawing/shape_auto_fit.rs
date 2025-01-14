// a:spAutoFit
use std::io::Cursor;

use quick_xml::{Reader, Writer, events::BytesStart};

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct ShapeAutoFit {}
impl ShapeAutoFit {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _: &BytesStart) {}

    #[inline]
    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:spAutoFit
        write_start_tag(writer, "a:spAutoFit", vec![], true);
    }
}
