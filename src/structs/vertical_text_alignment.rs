// vertAlign
use super::VerticalAlignmentRunValues;
use super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct VerticalTextAlignment {
    pub(crate) val: EnumValue<VerticalAlignmentRunValues>,
}
impl VerticalTextAlignment {
    pub fn get_val(&self)-> &VerticalAlignmentRunValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:VerticalAlignmentRunValues)-> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"val") {
            Some(v) => {self.val.set_value_string(v);},
            None => {},
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // vertAlign
        if &self.val.has_value() == &true {
            write_start_tag(writer, "vertAlign", vec![
                ("val", &self.val.get_value_string()),
            ], true);
        }
    }
}
