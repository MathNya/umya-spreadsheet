// a:prstDash
use super::super::EnumValue;
use super::PresetLineDashValues;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct PresetDash {
    val: EnumValue<PresetLineDashValues>,
}
impl PresetDash {
    #[inline]
    pub fn get_val(&self) -> &PresetLineDashValues {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: PresetLineDashValues) -> &mut PresetDash {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:prstDash
        write_start_tag(
            writer,
            "a:prstDash",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
