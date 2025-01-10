// c:layoutTarget
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::EnumValue,
    LayoutTargetValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct LayoutTarget {
    val: EnumValue<LayoutTargetValues>,
}
impl LayoutTarget {
    #[must_use]
    pub fn get_val(&self) -> &LayoutTargetValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: LayoutTargetValues) -> &mut LayoutTarget {
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
        // c:hMode
        write_start_tag(
            writer,
            "c:hMode",
            vec![("val", self.val.get_value_string()).into()],
            true,
        );
    }
}
