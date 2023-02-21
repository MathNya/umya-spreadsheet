use super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PositiveFixedPercentageType {
    val: Int32Value,
}
impl PositiveFixedPercentageType {
    pub fn get_val(&self) -> &i32 {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: i32) -> &mut Self {
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

    pub(crate) fn write_to_shade(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:shade")
    }

    pub(crate) fn write_to_alpha(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:alpha")
    }

    pub(crate) fn write_to_tint(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:tint")
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tab_name: &str) {
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let val = self.val.get_value_string();
            attributes.push(("val", &val));
            write_start_tag(writer, tab_name, attributes, true);
        }
    }
}
