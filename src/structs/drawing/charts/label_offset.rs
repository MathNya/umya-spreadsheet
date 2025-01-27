// c:lblOffset
use std::io::Cursor;

use quick_xml::{Reader, Writer, events::BytesStart};

use super::super::super::UInt16Value;
use crate::{reader::driver::get_attribute, writer::driver::write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct LabelOffset {
    val: UInt16Value,
}
impl LabelOffset {
    #[must_use]
    pub fn get_val(&self) -> u16 {
        self.val.value()
    }

    pub fn set_val(&mut self, value: u16) -> &mut LabelOffset {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:lblOffset
        write_start_tag(
            writer,
            "c:lblOffset",
            vec![("val", &self.val.value_string()).into()],
            true,
        );
    }
}
