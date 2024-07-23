// oddHeader
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OddHeader {
    value: StringValue,
}

impl OddHeader {
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn _get_hash_code(&self) -> String {
        format!("{:x}", md5::Md5::digest(self.get_value()))
    }

    pub(crate) fn has_param(&self) -> bool {
        self.value.has_value()
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
                if e.name().0 == b"oddHeader" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "oddHeader")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // oddHeader
            write_start_tag(writer, "oddHeader", vec![], false);
            write_text_node(writer, self.value.get_value_str());
            write_end_tag(writer, "oddHeader");
        }
    }
}
