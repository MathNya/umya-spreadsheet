// a:picLocks
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PictureLocks {
    no_change_aspect: bool,
}

impl PictureLocks {
    #[inline]
    #[must_use]
    pub fn get_no_change_aspect(&self) -> bool {
        self.no_change_aspect
    }

    #[inline]
    pub fn set_no_change_aspect(&mut self, value: bool) {
        self.no_change_aspect = value;
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"noChangeAspect") {
            if v == "1" {
                self.set_no_change_aspect(true);
            }
        }
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:picLocks
        let no_change_aspect = if self.no_change_aspect { "1" } else { "2" };
        write_start_tag(writer, "a:picLocks", vec![("noChangeAspect", no_change_aspect)], true);
    }
}
