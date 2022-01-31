// c:pt
use super::super::super::UInt32Value;
use super::NumericValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct StringPoint {
    index: UInt32Value,
    numeric_value: NumericValue,
}
impl StringPoint {
    pub fn get_index(&self) -> &u32 {
        &self.index.get_value()
    }

    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index.set_value(value);
        self
    }

    pub fn get_numeric_value(&self) -> &NumericValue {
        &self.numeric_value
    }

    pub fn get_numeric_value_mut(&mut self) -> &mut NumericValue {
        &mut self.numeric_value
    }

    pub fn set_numeric_value(&mut self, value: NumericValue) -> &mut Self {
        self.numeric_value = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        &mut self
            .index
            .set_value_string(get_attribute(e, b"idx").unwrap());

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"c:v" => {
                        &mut self.numeric_value.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"c:pt" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:pt"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:pt
        write_start_tag(
            writer,
            "c:pt",
            vec![("idx", &self.index.get_value_string())],
            false,
        );

        // c:v
        &self.numeric_value.write_to(writer);

        write_end_tag(writer, "c:pt");
    }
}
