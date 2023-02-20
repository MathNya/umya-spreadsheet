// c:pt
use super::NumericValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct StringPoint {
    numeric_value: NumericValue,
}
impl StringPoint {
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
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
                    b"c:v" => {
                        self.numeric_value._set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
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

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, index: &u32) {
        // c:pt
        let index_str = index.to_string();
        write_start_tag(writer, "c:pt", vec![("idx", index_str.as_str())], false);

        // c:v
        self.numeric_value._write_to(writer);

        write_end_tag(writer, "c:pt");
    }
}
