// c:v
use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use crate::{
    writer::driver::{write_end_tag, write_start_tag, write_text_node},
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct NumericValue {
    text: Box<str>,
}

impl NumericValue {
    #[must_use]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut NumericValue {
        self.text = value.into().into_boxed_str();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_text(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:v" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:v"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, &*self.text);
        write_end_tag(writer, "c:v");
    }
}
