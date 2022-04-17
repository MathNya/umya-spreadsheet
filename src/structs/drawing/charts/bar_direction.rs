// c:barDir
use super::super::super::EnumValue;
use super::BarDirectionValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BarDirection {
    val: EnumValue<BarDirectionValues>,
}
impl BarDirection {
    pub fn get_val(&self) -> &BarDirectionValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: BarDirectionValues) -> &mut BarDirection {
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
        // c:barDir
        write_start_tag(
            writer,
            "c:barDir",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
