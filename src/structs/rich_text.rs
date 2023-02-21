use super::TextElement;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::fmt::Write;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct RichText {
    rich_text_elements: Vec<TextElement>,
}
impl RichText {
    pub fn get_text(&self) -> Cow<'static, str> {
        let mut text = String::from("");
        for rich_text_elements in &self.rich_text_elements {
            text = format!("{}{}", text, rich_text_elements.get_text());
        }
        text.into()
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.rich_text_elements.clear();
        let mut text_element = TextElement::default();
        text_element.set_text(value);
        self.add_rich_text_elements(text_element);
        self
    }

    pub fn get_rich_text_elements(&self) -> &Vec<TextElement> {
        &self.rich_text_elements
    }

    pub fn get_rich_text_elements_mut(&mut self) -> &mut Vec<TextElement> {
        &mut self.rich_text_elements
    }

    pub fn set_rich_text_elements(&mut self, value: Vec<TextElement>) -> &mut Self {
        self.rich_text_elements = value;
        self
    }

    pub fn add_rich_text_elements(&mut self, value: TextElement) -> &mut Self {
        self.rich_text_elements.push(value);
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        let mut value = String::from("");
        for ele in &self.rich_text_elements {
            write!(value, "{}", ele.get_hash_code().as_str()).unwrap();
        }
        format!("{:x}", md5::Md5::digest(&value))
    }

    pub(crate) fn set_attributes_text<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"r" => {
                        let mut obj = TextElement::default();
                        obj.set_attributes(reader, e);
                        self.add_rich_text_elements(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"text" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "text"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to_none(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // none
        self.write_to(writer, "");
    }

    pub(crate) fn write_to_text(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // text
        self.write_to(writer, "text");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        if !tag_name.is_empty() {
            write_start_tag(writer, tag_name, vec![], false);
        }

        // r
        for obj in &self.rich_text_elements {
            obj.write_to(writer);
        }

        if !tag_name.is_empty() {
            write_end_tag(writer, tag_name);
        }
    }
}
