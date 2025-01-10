// c:crosses
use super::super::super::EnumValue;
use super::CrossesValues;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Crosses {
    val: EnumValue<CrossesValues>,
}
impl Crosses {
    pub fn get_val(&self) -> &CrossesValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: CrossesValues) -> &mut Crosses {
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
        // c:crosses
        write_start_tag(
            writer,
            "c:crosses",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
