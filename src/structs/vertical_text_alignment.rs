// vertAlign
use super::EnumValue;
use super::VerticalAlignmentRunValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VerticalTextAlignment {
    pub(crate) val: EnumValue<VerticalAlignmentRunValues>,
}
impl VerticalTextAlignment {
    pub fn get_val(&self) -> &VerticalAlignmentRunValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: VerticalAlignmentRunValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"val") {
            Some(v) => {
                self.val.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // vertAlign
        if &self.val.has_value() == &true {
            write_start_tag(
                writer,
                "vertAlign",
                vec![("val", self.val.get_value_string())],
                true,
            );
        }
    }
}
