// c:baseTimeUnit
use super::super::super::EnumValue;
use super::TimeUnitValues;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct BaseTimeUnit {
    val: EnumValue<TimeUnitValues>,
}
impl BaseTimeUnit {
    pub fn get_val(&self) -> &TimeUnitValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: TimeUnitValues) -> &mut Self {
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
        // c:baseTimeUnit
        write_start_tag(
            writer,
            "c:baseTimeUnit",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
