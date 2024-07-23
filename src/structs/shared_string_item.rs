// si
use super::PhoneticRun;
use super::RichText;
use super::Text;
use super::TextElement;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::hash::Hasher;
use std::io::Cursor;
use writer::driver::*;
extern crate ahash;
use self::ahash::AHasher;

#[derive(Clone, Default, Debug)]
pub(crate) struct SharedStringItem {
    text: Option<Text>,
    rich_text: Option<RichText>,
}

impl SharedStringItem {
    pub(crate) fn get_text(&self) -> Option<&Text> {
        self.text.as_ref()
    }

    pub(crate) fn _get_text_mut(&mut self) -> Option<&mut Text> {
        self.text.as_mut()
    }

    pub(crate) fn set_text(&mut self, value: Text) -> &mut Self {
        self.text = Some(value);
        self
    }

    pub(crate) fn _remove_text(&mut self) -> &mut Self {
        self.text = None;
        self
    }

    pub(crate) fn get_rich_text(&self) -> Option<&RichText> {
        self.rich_text.as_ref()
    }

    pub(crate) fn get_rich_text_mut(&mut self) -> Option<&mut RichText> {
        self.rich_text.as_mut()
    }

    pub(crate) fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.rich_text = Some(value);
        self
    }

    pub(crate) fn _remove_rich_text(&mut self) -> &mut Self {
        self.rich_text = None;
        self
    }

    pub(crate) fn get_hash_u64(&self) -> u64 {
        let mut h = AHasher::default();
        let content = format!(
            "{}{}",
            self.text
                .as_ref()
                .map_or(String::from("NONE"), |v| v.get_hash_code()),
            self.rich_text
                .as_ref()
                .map_or(String::from("NONE"), |v| v.get_hash_code())
        );
        h.write(content.as_bytes());
        h.finish()
    }

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
                        let mut obj = PhoneticRun::default();
                        obj.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"si" {
                    if !vec_text_element.is_empty() {
                        let mut obj = RichText::default();
                        obj.set_rich_text_elements(vec_text_element);
                        self.set_rich_text(obj);
                    }
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "si")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // si
        write_start_tag(writer, "si", vec![], false);

        // t
        if let Some(v) = &self.text {
            v.write_to(writer);
        }

        // r
        if let Some(v) = &self.rich_text {
            v.write_to_none(writer);
        }

        write_start_tag(writer, "phoneticPr", vec![("fontId", "1")], true);

        write_end_tag(writer, "si");
    }
}
