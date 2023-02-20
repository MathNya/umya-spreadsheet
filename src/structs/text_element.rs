// r
use super::Font;
use super::Text;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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

    pub fn get_run_properties(&self) -> &Option<Font> {
        &self.run_properties
    }

    pub fn get_run_properties_mut(&mut self) -> &mut Font {
        match &self.run_properties {
            Some(_) => return self.run_properties.as_mut().unwrap(),
            None => {}
        }
        self.set_run_properties(Font::get_defalut_value());
        self.run_properties.as_mut().unwrap()
    }

    pub(crate) fn get_run_properties_crate(&mut self) -> &mut Option<Font> {
        &mut self.run_properties
    }

    pub fn set_run_properties(&mut self, value: Font) -> &mut Self {
        self.run_properties = Some(value);
        self
    }

    pub fn get_font(&self) -> &Option<Font> {
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"r" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "r"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // r
        write_start_tag(writer, "r", vec![], false);

        // rPr
        match &self.run_properties {
            Some(v) => {
                v.write_to_rpr(writer);
            }
            None => {}
        }

        // t
        let _ = &self.text.write_to(writer);

        write_end_tag(writer, "r");
    }
}
