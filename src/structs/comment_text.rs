// text
use super::PhoneticRun;
use super::RichText;
use super::Text;
use super::TextElement;
use crate::writer::driver::{write_end_tag, write_start_tag};
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CommentText {
    text: Option<Text>,
    rich_text: Option<RichText>,
}

impl CommentText {
    #[inline]
    pub(crate) fn get_text(&self) -> Option<&Text> {
        self.text.as_ref()
    }

    #[inline]
    pub(crate) fn get_text_mut(&mut self) -> Option<&mut Text> {
        self.text.as_mut()
    }

    #[inline]
    pub(crate) fn set_text(&mut self, value: Text) -> &mut Self {
        self.text = Some(value);
        self
    }

    #[inline]
    pub(crate) fn remove_text(&mut self) -> &mut Self {
        self.text = None;
        self
    }

    #[inline]
    pub(crate) fn get_rich_text(&self) -> Option<&RichText> {
        self.rich_text.as_ref()
    }

    #[inline]
    pub(crate) fn get_rich_text_mut(&mut self) -> Option<&mut RichText> {
        self.rich_text.as_mut()
    }

    #[inline]
    pub(crate) fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.rich_text = Some(value);
        self
    }

    #[inline]
    pub(crate) fn remove_rich_text(&mut self) -> &mut Self {
        self.rich_text = None;
        self
    }

    #[inline]
    pub fn set_text_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let mut obj = Text::default();
        obj.set_value(value);
        self.set_text(obj);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut vec_text_element: Vec<TextElement> = Vec::new();

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"t" => {
                        let mut obj = Text::default();
                        obj.set_attributes(reader, e);
                        self.set_text(obj);
                    }
                    b"r" => {
                        let mut obj = TextElement::default();
                        obj.set_attributes(reader, e);
                        vec_text_element.push(obj);
                    }
                    b"rPh" => {
                        PhoneticRun::set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"text" {
                    if !vec_text_element.is_empty() {
                        let mut obj = RichText::default();
                        obj.set_rich_text_elements(vec_text_element);
                        self.set_rich_text(obj);
                    }
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "text")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // text
        write_start_tag(writer, "text", vec![], false);

        // t
        if let Some(v) = &self.text {
            v.write_to(writer);
        }

        // r
        if let Some(v) = &self.rich_text {
            v.write_to(writer);
        }

        write_end_tag(writer, "text");
    }
}
