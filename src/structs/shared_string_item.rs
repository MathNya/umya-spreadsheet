// si
use super::PhoneticRun;
use super::RichText;
use super::Text;
use super::TextElement;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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
    pub(crate) fn get_text(&self) -> &Option<Text> {
        &self.text
    }

    pub(crate) fn _get_text_mut(&mut self) -> &mut Option<Text> {
        &mut self.text
    }

    pub(crate) fn set_text(&mut self, value: Text) -> &mut Self {
        self.text = Some(value);
        self
    }

    pub(crate) fn _remove_text(&mut self) -> &mut Self {
        self.text = None;
        self
    }

    pub(crate) fn get_rich_text(&self) -> &Option<RichText> {
        &self.rich_text
    }

    pub(crate) fn get_rich_text_mut(&mut self) -> &mut Option<RichText> {
        &mut self.rich_text
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
            match &self.text {
                Some(v) => {
                    v.get_hash_code()
                }
                None => {
                    String::from("NONE")
                }
            },
            match &self.rich_text {
                Some(v) => {
                    v.get_hash_code()
                }
                None => {
                    String::from("NONE")
                }
            }
        );
        h.write(content.as_bytes());
        h.finish()
    }

    pub(crate) fn _get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                match &self.text {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        String::from("NONE")
                    }
                },
                match &self.rich_text {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        String::from("NONE")
                    }
                }
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut vec_text_element: Vec<TextElement> = Vec::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"si" => {
                        if !vec_text_element.is_empty() {
                            let mut obj = RichText::default();
                            obj.set_rich_text_elements(vec_text_element);
                            self.set_rich_text(obj);
                        }
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "si"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // si
        write_start_tag(writer, "si", vec![], false);

        // t
        match &self.text {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // r
        match &self.rich_text {
            Some(v) => {
                v.write_to_none(writer);
            }
            None => {}
        }

        write_start_tag(writer, "phoneticPr", vec![("fontId", "1")], true);

        write_end_tag(writer, "si");
    }
}
