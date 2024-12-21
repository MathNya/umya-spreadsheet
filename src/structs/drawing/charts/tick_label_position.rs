// c:tickLblPos
use super::super::super::EnumValue;
use super::TickLabelPositionValues;
use crate::reader::driver::get_attribute;
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct TickLabelPosition {
    val: EnumValue<TickLabelPositionValues>,
}
impl TickLabelPosition {
    #[must_use]
    pub fn get_val(&self) -> &TickLabelPositionValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: TickLabelPositionValues) -> &mut TickLabelPosition {
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
        // c:tickLblPos
        write_start_tag(
            writer,
            "c:tickLblPos",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
