// c:xMode
use super::LayoutModeValues;
use super::super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct LeftMode {
    val: EnumValue<LayoutModeValues>,
}
impl LeftMode {
    pub fn get_val(&self)-> &LayoutModeValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:LayoutModeValues)-> &mut LeftMode {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader:&mut Reader<R>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:xMode
        write_start_tag(writer, "c:xMode", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}
