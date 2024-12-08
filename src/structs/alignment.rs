// alignment
use super::BooleanValue;
use super::EnumValue;
use super::HorizontalAlignmentValues;
use super::UInt32Value;
use super::VerticalAlignmentValues;
use md5::Digest;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Default, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Alignment {
    horizontal: EnumValue<HorizontalAlignmentValues>,
    vertical: EnumValue<VerticalAlignmentValues>,
    wrap_text: BooleanValue,
    text_rotation: UInt32Value,
}

impl Alignment {
    #[inline]
    pub fn get_horizontal(&self) -> &HorizontalAlignmentValues {
        self.horizontal.get_value()
    }

    #[inline]
    pub fn set_horizontal(&mut self, value: HorizontalAlignmentValues) {
        self.horizontal.set_value(value);
    }

    #[inline]
    pub fn get_vertical(&self) -> &VerticalAlignmentValues {
        self.vertical.get_value()
    }

    #[inline]
    pub fn set_vertical(&mut self, value: VerticalAlignmentValues) {
        self.vertical.set_value(value);
    }

    #[inline]
    pub fn get_wrap_text(&self) -> &bool {
        self.wrap_text.get_value()
    }

    #[inline]
    pub fn set_wrap_text(&mut self, value: bool) {
        self.wrap_text.set_value(value);
    }

    #[inline]
    pub fn get_text_rotation(&self) -> &u32 {
        self.text_rotation.get_value()
    }

    #[inline]
    pub fn set_text_rotation(&mut self, value: u32) {
        self.text_rotation.set_value(value);
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}",
                &self.horizontal.get_hash_string(),
                &self.vertical.get_hash_string(),
                &self.wrap_text.get_hash_string(),
                &self.text_rotation.get_hash_string(),
            ))
        )
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, horizontal, "horizontal");
        set_string_from_xml!(self, e, vertical, "vertical");
        set_string_from_xml!(self, e, wrap_text, "wrapText");
        set_string_from_xml!(self, e, text_rotation, "textRotation");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // alignment
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.horizontal.has_value() {
            attributes.push(("horizontal", self.horizontal.get_value_string()));
        }
        if self.vertical.has_value() {
            attributes.push(("vertical", self.vertical.get_value_string()));
        }
        if self.wrap_text.has_value() {
            attributes.push(("wrapText", self.wrap_text.get_value_string()));
        }
        let text_rotation = self.text_rotation.get_value_string();
        if self.text_rotation.has_value() {
            attributes.push(("textRotation", &text_rotation));
        }
        write_start_tag(writer, "alignment", attributes, true);
    }
}
