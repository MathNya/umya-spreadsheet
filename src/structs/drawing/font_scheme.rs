// a:fontScheme
use super::super::StringValue;
use super::FontCollectionType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct FontScheme {
    name: StringValue,
    major_font: FontCollectionType,
    minor_font: FontCollectionType,
}

impl FontScheme {
    pub fn get_name(&self) -> &str {
        self.name.get_value_str()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    pub fn get_major_font(&self) -> &FontCollectionType {
        &self.major_font
    }

    pub fn get_major_font_mut(&mut self) -> &mut FontCollectionType {
        &mut self.major_font
    }

    pub fn set_major_font(&mut self, value: FontCollectionType) -> &mut Self {
        self.major_font = value;
        self
    }

    pub fn get_minor_font(&self) -> &FontCollectionType {
        &self.minor_font
    }

    pub fn get_minor_font_mut(&mut self) -> &mut FontCollectionType {
        &mut self.minor_font
    }

    pub fn set_minor_font(&mut self, value: FontCollectionType) -> &mut Self {
        self.minor_font = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"name") {
            self.name.set_value(v);
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:majorFont" => {
                    self.major_font.set_attributes(reader, e);
                }
                b"a:minorFont" => {
                    self.minor_font.set_attributes(reader, e);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:fontScheme" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:fontScheme")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fontScheme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_str()));
        }
        write_start_tag(writer, "a:fontScheme", attributes, false);

        // a:majorFont
        self.major_font.write_to_major_font(writer);

        // a:minorFont
        self.minor_font.write_to_minor_font(writer);

        write_end_tag(writer, "a:fontScheme");
    }
}
