// a:prstDash
use super::PresetLineDashValues;
use super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct PresetDash {
    val:EnumValue<PresetLineDashValues>,
}
impl PresetDash {
    pub fn get_val(&self)-> &PresetLineDashValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:PresetLineDashValues)-> &mut PresetDash {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:prstDash
        write_start_tag(writer, "a:prstDash", vec![
            ("val", &self.val.get_value_string())
        ], true);
    }
}