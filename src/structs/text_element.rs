// r
use super::Font;
use super::Text;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct TextElement {
    text: Text,
    run_properties: Option<Font>,
}

impl TextElement {
    pub fn get_text(&self) -> &str {
        self.text.get_value()
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text.set_value(value);
        self
    }

    pub fn get_run_properties(&self) -> Option<&Font> {
        self.run_properties.as_ref()
    }

    pub fn get_run_properties_mut(&mut self) -> &mut Font {
        if self.run_properties.is_some() {
            return self.run_properties.as_mut().unwrap();
        }
        self.set_run_properties(Font::get_default_value());
        self.run_properties.as_mut().unwrap()
    }

    pub(crate) fn get_run_properties_crate(&mut self) -> Option<&mut Font> {
        self.run_properties.as_mut()
    }

    pub fn set_run_properties(&mut self, value: Font) -> &mut Self {
        self.run_properties = Some(value);
        self
    }

    pub fn get_font(&self) -> Option<&Font> {
        self.get_run_properties()
    }

    pub fn get_font_mut(&mut self) -> &mut Font {
        self.get_run_properties_mut()
    }

    pub fn set_font(&mut self, value: Font) -> &mut Self {
        self.set_run_properties(value)
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                &self.text.get_value(),
                match &self.run_properties {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"t" => {
                        let mut obj = Text::default();
                        obj.set_attributes(reader, e);
                        self.text = obj;
                    }
                    b"rPr" => {
                        let mut obj = Font::default();
                        obj.set_attributes(reader, e);
                        self.set_run_properties(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"r" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "r")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // r
        write_start_tag(writer, "r", vec![], false);

        // rPr
        if let Some(v) = &self.run_properties {
            v.write_to_rpr(writer);
        }

        // t
        self.text.write_to(writer);

        write_end_tag(writer, "r");
    }
}
