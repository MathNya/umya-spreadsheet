use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::StringValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct TextFontType {
    typeface:     StringValue,
    pitch_family: StringValue,
    charset:      StringValue,
    panose:       StringValue,
}

impl TextFontType {
    #[must_use]
    pub fn get_typeface(&self) -> &str {
        self.typeface.get_value_str()
    }

    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.typeface.set_value(value.into());
        self
    }

    #[must_use]
    pub fn get_pitch_family(&self) -> &str {
        self.pitch_family.get_value_str()
    }

    pub fn set_pitch_family<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.pitch_family.set_value(value.into());
        self
    }

    #[must_use]
    pub fn get_charset(&self) -> &str {
        self.charset.get_value_str()
    }

    pub fn set_charset<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.charset.set_value(value.into());
        self
    }

    #[must_use]
    pub fn get_panose(&self) -> &str {
        self.panose.get_value_str()
    }

    pub fn set_panose<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.panose.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"typeface") {
            self.set_typeface(v);
        }
        if let Some(v) = get_attribute(e, b"pitchFamily") {
            self.set_pitch_family(v);
        }
        if let Some(v) = get_attribute(e, b"charset") {
            self.set_charset(v);
        }
        if let Some(v) = get_attribute(e, b"panose") {
            self.set_panose(v);
        }
    }

    pub(crate) fn write_to_latin(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:latin");
    }

    pub(crate) fn write_to_cs(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:cs");
    }

    pub(crate) fn write_to_ea(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:ea");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tab_name: &str) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.typeface.has_value() {
            attributes.push(("typeface", self.typeface.get_value_str()).into());
        }
        if self.pitch_family.has_value() {
            attributes.push(("pitchFamily", self.pitch_family.get_value_str()).into());
        }
        if self.charset.has_value() {
            attributes.push(("charset", self.charset.get_value_str()).into());
        }
        if self.panose.has_value() {
            attributes.push(("panose", self.panose.get_value_str()).into());
        }
        write_start_tag(writer, tab_name, attributes, true);
    }
}
