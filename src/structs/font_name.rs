// name
use super::StringValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FontName {
    pub(crate) val: StringValue,
}
impl FontName {
    pub fn get_val(&self) -> &str {
        self.val.get_value()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
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

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // name, rFont
        if &self.val.has_value() == &true {
            write_start_tag(
                writer,
                tag_name,
                vec![("val", self.val.get_value_string())],
                true,
            );
        }
    }
}
