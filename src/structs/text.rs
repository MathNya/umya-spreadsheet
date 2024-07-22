// t
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Text {
    value: String,
}

impl Text {
    pub(crate) fn get_value(&self) -> &str {
        &self.value
    }

    pub(crate) fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = value.into();
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!("{:x}", md5::Md5::digest(&self.value))
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_value(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"t" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "t")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // t
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.value.starts_with(|c: char| c.is_whitespace())
            || self.value.ends_with(|c: char| c.is_whitespace())
        {
            attributes.push(("xml:space", "preserve"));
        }
        write_start_tag(writer, "t", attributes, false);
        write_text_node(writer, &self.value);
        write_end_tag(writer, "t");
    }
}
