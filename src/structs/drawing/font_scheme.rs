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
        self.name.get_value()
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
        match get_attribute(e, b"name") {
            Some(v) => {
                self.name.set_value(v);
            }
            _ => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:majorFont" => {
                        self.major_font.set_attributes(reader, e);
                    }
                    b"a:minorFont" => {
                        self.minor_font.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:fontScheme" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:fontScheme"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fontScheme
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.name.has_value() {
            attributes.push(("name", self.name.get_value_string()));
        }
        write_start_tag(writer, "a:fontScheme", attributes, false);

        // a:majorFont
        let _ = &self.major_font.write_to_major_font(writer);

        // a:minorFont
        let _ = &self.minor_font.write_to_minor_font(writer);

        write_end_tag(writer, "a:fontScheme");
    }
}
