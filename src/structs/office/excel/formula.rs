// xm:f
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use std::vec;
use structs::Address;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct Formula {
    value: Address,
}
impl Formula {
    pub fn get_value(&self) -> &Address {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Address {
        &mut self.value
    }

    pub fn set_value(&mut self, value: Address) -> &mut Self {
        self.value = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut value: String = String::from("");
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    value = e.unescape().unwrap().to_string();
                }
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xm:f" => {
                        let mut obj = Address::default();
                        obj.set_address(value);
                        self.value = obj;
                        value = String::from("");
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "xm:f"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:f", vec![], false);
        write_text_node(writer, &self.value.get_address());
        write_end_tag(writer, "xm:f");
    }
}
