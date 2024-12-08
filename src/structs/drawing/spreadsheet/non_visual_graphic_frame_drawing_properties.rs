// xdr:cNvGraphicFramePr
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualGraphicFrameDrawingProperties {}
impl NonVisualGraphicFrameDrawingProperties {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvGraphicFramePr
        write_start_tag(writer, "xdr:cNvGraphicFramePr", vec![], true);
    }
}
