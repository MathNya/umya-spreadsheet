// u
use super::EnumTrait;
use super::EnumValue;
use super::UnderlineValues;
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Underline {
    pub(crate) val: EnumValue<UnderlineValues>,
}

impl Underline {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &UnderlineValues {
        if self.val.has_value() {
            return self.val.get_value();
        }
        &UnderlineValues::None
    }

    #[inline]
    pub fn set_val(&mut self, value: UnderlineValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_val(UnderlineValues::default());
        set_string_from_xml!(self, e, val, "val");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // u
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            if self.val.get_value_string() != UnderlineValues::Single.get_value_string() {
                attributes.push(("val", self.val.get_value_string()));
            }
            write_start_tag(writer, "u", attributes, true);
        }
    }
}
