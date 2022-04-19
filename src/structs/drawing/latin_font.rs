// a:latin
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LatinFont {
    typeface: StringValue,
    pitch_family: StringValue,
    charset: StringValue,
}
impl LatinFont {
    pub fn get_typeface(&self) -> &str {
        self.typeface.get_value()
    }

    pub fn set_typeface<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.typeface.set_value(value.into());
        self
    }

    pub fn get_pitch_family(&self) -> &str {
        &self.pitch_family.get_value()
    }

    pub fn set_pitch_family<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.pitch_family.set_value(value.into());
        self
    }

    pub fn get_charset(&self) -> &str {
        self.charset.get_value()
    }

    pub fn set_charset<S: Into<String>>(&mut self, value: S) -> &mut LatinFont {
        self.charset.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"typeface") {
            Some(v) => {
                self.set_typeface(v);
            }
            None => {}
        }
        match get_attribute(e, b"pitchFamily") {
            Some(v) => {
                self.set_pitch_family(v);
            }
            None => {}
        }
        match get_attribute(e, b"charset") {
            Some(v) => {
                self.set_charset(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:latin
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.typeface.has_value() {
            attributes.push(("typeface", &self.typeface.get_value_string()));
        }
        if self.pitch_family.has_value() {
            attributes.push(("pitchFamily", &self.pitch_family.get_value_string()));
        }
        if self.charset.has_value() {
            attributes.push(("charset", &self.charset.get_value_string()));
        }
        write_start_tag(writer, "a:latin", attributes, true);
    }
}
