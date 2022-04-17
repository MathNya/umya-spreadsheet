// c:lblAlgn
use super::super::super::EnumValue;
use super::LabelAlignmentValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LabelAlignment {
    val: EnumValue<LabelAlignmentValues>,
}
impl LabelAlignment {
    pub fn get_val(&self) -> &LabelAlignmentValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: LabelAlignmentValues) -> &mut LabelAlignment {
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
        // c:lblAlgn
        write_start_tag(
            writer,
            "c:lblAlgn",
            vec![("val", self.val.get_value_string())],
            true,
        );
    }
}
