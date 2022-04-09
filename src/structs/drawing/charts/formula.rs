// c:f
use super::super::super::Address;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Formula {
    address: Address,
}
impl Formula {
    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    pub fn get_address_str(&self) -> String {
        self.address.get_address()
    }

    pub fn set_address(&mut self, value: Address) -> &mut Formula {
        self.address = value;
        self
    }

    pub fn set_address_str<S: Into<String>>(&mut self, value: S) -> &mut Formula {
        self.address.set_address(value);
        self
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
                    self.set_address_str(e.unescape_and_decode(reader).unwrap());
                }
                Ok(Event::End(ref e)) => match e.name() {
                    b"c:f" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:f"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:f
        write_start_tag(writer, "c:f", vec![], false);
        write_text_node_no_escape(writer, self.get_address_str());
        write_end_tag(writer, "c:f");
    }
}
