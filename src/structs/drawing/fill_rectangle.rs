// a:fillRect
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::writer::driver::write_start_tag;

#[derive(Clone, Default, Debug)]
pub struct FillRectangle {
    bottom: usize,
    left: usize,
    right: usize,
    top: usize,
}
impl FillRectangle {
    #[inline]
    #[must_use]
    pub fn get_bottom(&self) -> usize {
        self.bottom
    }

    #[inline]
    pub fn set_bottom(&mut self, value: usize) {
        self.bottom = value;
    }

    #[inline]
    #[must_use]
    pub fn get_left(&self) -> usize {
        self.left
    }

    #[inline]
    pub fn set_left(&mut self, value: usize) {
        self.left = value;
    }

    #[inline]
    #[must_use]
    pub fn get_right(&self) -> usize {
        self.right
    }

    #[inline]
    pub fn set_right(&mut self, value: usize) {
        self.right = value;
    }

    #[inline]
    #[must_use]
    pub fn get_top(&self) -> usize {
        self.top
    }

    #[inline]
    pub fn set_top(&mut self, value: usize) {
        self.top = value;
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(_reader: &mut Reader<R>, _e: &BytesStart) {}

    #[inline]
    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fillRect
        write_start_tag(writer, "a:fillRect", vec![], true);
    }
}
