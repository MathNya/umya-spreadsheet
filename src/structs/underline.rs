// u
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    EnumTrait,
    EnumValue,
    UnderlineValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Underline {
    pub(crate) val: EnumValue<UnderlineValues>,
}

impl Underline {
    #[inline]
    #[must_use]
    pub fn val(&self) -> &UnderlineValues {
        if self.val.has_value() {
            return self.val.value();
        }
        &UnderlineValues::None
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_val(&self) -> &UnderlineValues {
        self.val()
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
            let mut attributes: crate::structs::AttrCollection = Vec::new();
            if self.val.value_string() != UnderlineValues::Single.value_string() {
                attributes.push(("val", self.val.value_string()).into());
            }
            write_start_tag(writer, "u", attributes, true);
        }
    }
}
