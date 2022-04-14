// t
use md5::Digest;
// use onig::*;
use fancy_regex::Regex;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Text(e)) => {
                    self.set_value(e.unescape_and_decode(reader).unwrap());
                }
                Ok(Event::End(ref e)) => match e.name() {
                    b"t" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "t"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // t
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"^(\s|　)"#).unwrap();
        }
        if RE.find(&self.value).ok().flatten().is_some() {
            attributes.push(("xml:space", "preserve"));
        }
        write_start_tag(writer, "t", attributes, false);
        write_text_node(writer, &self.value);
        write_end_tag(writer, "t");
    }
}
