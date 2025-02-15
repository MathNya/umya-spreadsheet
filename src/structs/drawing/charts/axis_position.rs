// c:axPos
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::EnumValue,
    AxisPositionValues,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct AxisPosition {
    val: EnumValue<AxisPositionValues>,
}
impl AxisPosition {
    #[must_use]
    pub fn val(&self) -> &AxisPositionValues {
        self.val.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> &AxisPositionValues {
        self.val()
    }

    pub fn set_val(&mut self, value: AxisPositionValues) -> &mut AxisPosition {
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
        // c:axPos
        write_start_tag(
            writer,
            "c:axPos",
            vec![("val", self.val.value_string()).into()],
            true,
        );
    }
}
