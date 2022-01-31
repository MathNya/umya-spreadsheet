// a:fillRect
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct FillRectangle {
    bottom: usize,
    left: usize,
    right: usize,
    top: usize,
}
impl FillRectangle {
    pub fn get_bottom(&self) -> &usize {
        &self.bottom
    }

    pub fn set_bottom(&mut self, value: usize) {
        self.bottom = value;
    }

    pub fn get_left(&self) -> &usize {
        &self.left
    }

    pub fn set_left(&mut self, value: usize) {
        self.left = value;
    }

    pub fn get_right(&self) -> &usize {
        &self.right
    }

    pub fn set_right(&mut self, value: usize) {
        self.right = value;
    }

    pub fn get_top(&self) -> &usize {
        &self.top
    }

    pub fn set_top(&mut self, value: usize) {
        self.top = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fillRect
        write_start_tag(writer, "a:fillRect", vec![], true);
    }
}
