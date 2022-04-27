// sz
use super::DoubleValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct FontSize {
    pub(crate) val: DoubleValue,
}
impl FontSize {
    pub fn get_val(&self) -> &f64 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: f64) -> &mut Self {
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
        // sz
        if &self.val.has_value() == &true {
            write_start_tag(
                writer,
                "sz",
                vec![("val", &self.val.get_value_string())],
                true,
            );
        }
    }
}
