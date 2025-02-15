// c:lblAlgn
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::EnumValue,
    LabelAlignmentValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct LabelAlignment {
    val: EnumValue<LabelAlignmentValues>,
}
impl LabelAlignment {
    #[must_use]
    pub fn get_val(&self) -> &LabelAlignmentValues {
        self.val.value()
    }

    pub fn set_val(&mut self, value: LabelAlignmentValues) -> &mut LabelAlignment {
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
        // c:lblAlgn
        write_start_tag(
            writer,
            "c:lblAlgn",
            vec![("val", self.val.value_string()).into()],
            true,
        );
    }
}
