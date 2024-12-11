use super::TextElement;
use crate::reader::driver::*;
use crate::writer::driver::*;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::fmt::Write;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct RichText {
    rich_text_elements: ThinVec<TextElement>,
}

impl RichText {
    #[inline]
    pub fn get_text(&self) -> Cow<'static, str> {
        let mut text = String::from("");
        for rich_text_elements in &self.rich_text_elements {
            text = format!("{}{}", text, rich_text_elements.get_text());
        }
        text.into()
    }

    #[inline]
    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.rich_text_elements.clear();
        let mut text_element = TextElement::default();
        text_element.set_text(value);
        self.add_rich_text_elements(text_element);
        self
    }

    #[inline]
    pub fn get_rich_text_elements(&self) -> &[TextElement] {
        &self.rich_text_elements
    }

    #[inline]
    pub fn get_rich_text_elements_mut(&mut self) -> &mut ThinVec<TextElement> {
        &mut self.rich_text_elements
    }

    #[inline]
    pub fn set_rich_text_elements(&mut self, value: impl Into<ThinVec<TextElement>>) -> &mut Self {
        self.rich_text_elements = value.into();
        self
    }

    #[inline]
    pub fn add_rich_text_elements(&mut self, value: TextElement) -> &mut Self {
        self.rich_text_elements.push(value);
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        let mut value = String::from("");
        for ele in &self.rich_text_elements {
            write!(value, "{}", ele.get_hash_code());
        }
        format!("{:x}", md5::Md5::digest(&value))
    }

    pub(crate) fn set_attributes_text<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"r" {
                    let mut obj = TextElement::default();
                    obj.set_attributes(reader, e);
                    self.add_rich_text_elements(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"text" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "text")
        );
    }

    #[inline]
    pub(crate) fn write_to_none(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // none
        self.write_to(writer, "");
    }

    #[inline]
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
