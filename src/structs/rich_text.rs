use std::{
    borrow::Cow,
    fmt::Write,
    io::Cursor,
};

use md5::Digest;
use quick_xml::Writer;

use super::TextElement;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct RichText {
    rich_text_elements: Vec<TextElement>,
}

impl RichText {
    #[inline]
    #[must_use]
    pub fn text(&self) -> Cow<'static, str> {
        let mut text = String::new();
        for rich_text_elements in &self.rich_text_elements {
            text = format!("{}{}", text, rich_text_elements.get_text());
        }
        text.into()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use text()")]
    pub fn get_text(&self) -> Cow<'static, str> {
        self.text()
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
    #[must_use]
    pub fn rich_text_elements(&self) -> &[TextElement] {
        &self.rich_text_elements
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rich_text_elements()")]
    pub fn get_rich_text_elements(&self) -> &[TextElement] {
        self.rich_text_elements()
    }

    #[inline]
    pub fn rich_text_elements_mut(&mut self) -> &mut Vec<TextElement> {
        &mut self.rich_text_elements
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use rich_text_elements_mut()")]
    pub fn get_rich_text_elements_mut(&mut self) -> &mut Vec<TextElement> {
        self.rich_text_elements_mut()
    }

    #[inline]
    pub fn set_rich_text_elements(&mut self, value: impl Into<Vec<TextElement>>) -> &mut Self {
        self.rich_text_elements = value.into();
        self
    }

    #[inline]
    pub fn add_rich_text_elements(&mut self, value: TextElement) -> &mut Self {
        self.rich_text_elements.push(value);
        self
    }

    pub(crate) fn hash_code(&self) -> String {
        let mut value = String::new();
        for ele in &self.rich_text_elements {
            write!(value, "{}", ele.get_hash_code()).unwrap();
        }
        format!("{:x}", md5::Md5::digest(&value))
    }

    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // r
        for obj in &self.rich_text_elements {
            obj.write_to(writer);
        }
    }
}
