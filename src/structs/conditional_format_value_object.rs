use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    ConditionalFormatValueObjectValues,
    EnumValue,
    StringValue,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct ConditionalFormatValueObject {
    r#type: EnumValue<ConditionalFormatValueObjectValues>,
    val:    StringValue,
}

impl ConditionalFormatValueObject {
    #[inline]
    #[must_use]
    pub fn get_type(&self) -> &ConditionalFormatValueObjectValues {
        self.r#type.get_value()
    }

    #[inline]
    pub fn set_type(&mut self, value: ConditionalFormatValueObjectValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_val(&self) -> &str {
        self.val.value_str()
    }

    #[inline]
    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.val.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, r#type, "type");
        set_string_from_xml!(self, e, val, "val");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"cfvo" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cfvo")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cfvo
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let ctype = self.r#type.get_value_string();
        if self.r#type.has_value() {
            attributes.push(("type", ctype).into());
        }
        let val = self.val.value_str();
        if self.val.has_value() {
            attributes.push(("val", val).into());
        }

        write_start_tag(writer, "cfvo", attributes, true);
    }
}
