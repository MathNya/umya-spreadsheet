// a:sysClr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::super::{
        EnumValue,
        StringValue,
    },
    SystemColorValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct SystemColor {
    val:        EnumValue<SystemColorValues>,
    last_color: StringValue,
}

impl SystemColor {
    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &SystemColorValues {
        self.val.value()
    }

    #[inline]
    pub fn set_val(&mut self, value: SystemColorValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_last_color(&self) -> &str {
        self.last_color.value_str()
    }

    #[inline]
    pub fn set_last_color<S: Into<String>>(&mut self, value: S) {
        self.last_color.set_value(value.into());
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, val, "val");
        set_string_from_xml!(self, e, last_color, "lastClr");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srgbClr
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let val = self.val.value_string();
        if self.val.has_value() {
            attributes.push(("val", val).into());
        }
        let last_color = self.last_color.value_str();
        if self.last_color.has_value() {
            attributes.push(("lastClr", last_color).into());
        }
        write_start_tag(writer, "a:sysClr", attributes, true);
    }
}
