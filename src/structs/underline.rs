// u
use super::EnumTrait;
use super::EnumValue;
use super::UnderlineValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Underline {
    pub(crate) val: EnumValue<UnderlineValues>,
}
impl Underline {
    pub fn get_val(&self) -> &UnderlineValues {
        if self.val.has_value() {
            return self.val.get_value();
        }
        &UnderlineValues::None
    }

    pub fn set_val(&mut self, value: UnderlineValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(UnderlineValues::default());
        match get_attribute(e, b"val") {
            Some(v) => {
                self.val.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // u
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            if &self.val.get_value_string() != &UnderlineValues::Single.get_value_string() {
                attributes.push(("val", self.val.get_value_string()));
            }
            write_start_tag(writer, "u", attributes, true);
        }
    }
}
