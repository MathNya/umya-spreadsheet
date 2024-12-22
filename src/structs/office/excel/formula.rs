// xm:f
use std::io::Cursor;
use std::vec;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use crate::structs::Address;
use crate::writer::driver::{write_end_tag, write_start_tag, write_text_node};

#[derive(Default, Debug, Clone)]
pub struct Formula {
    value: Address,
}
impl Formula {
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> &Address {
        &self.value
    }

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut Address {
        &mut self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: Address) -> &mut Self {
        self.value = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    value = e.unescape().unwrap().to_string();
                }
                Ok(Event::End(ref e)) =>
                    if e.name().into_inner() == b"xm:f" {
                        let mut obj = Address::default();
                        obj.set_address(value);
                        self.value = obj;
                        return;
                    },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "xm:f"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:f", vec![], false);
        write_text_node(writer, self.value.get_address());
        write_end_tag(writer, "xm:f");
    }
}
