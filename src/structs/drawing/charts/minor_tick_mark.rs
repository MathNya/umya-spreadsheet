// c:minorTickMark
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::EnumValue,
    TickMarkValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct MinorTickMark {
    val: EnumValue<TickMarkValues>,
}
impl MinorTickMark {
    #[must_use]
    pub fn get_val(&self) -> &TickMarkValues {
        self.val.value()
    }

    pub fn set_val(&mut self, value: TickMarkValues) -> &mut MinorTickMark {
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
        // c:minorTickMark
        write_start_tag(
            writer,
            "c:minorTickMark",
            vec![("val", self.val.value_string()).into()],
            true,
        );
    }
}
