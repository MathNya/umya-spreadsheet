// xdr:cNvGraphicFramePr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct NonVisualGraphicFrameDrawingProperties {}
impl NonVisualGraphicFrameDrawingProperties {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _e: &BytesStart) {}

    #[inline]
    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvGraphicFramePr
        write_start_tag(writer, "xdr:cNvGraphicFramePr", vec![], true);
    }
}
