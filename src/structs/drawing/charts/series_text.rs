// c:tx
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SeriesText {
    value: StringValue,
}
impl SeriesText {
    pub fn get_value(&self) -> &str {
        self.value.get_value()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
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
                Ok(Event::Text(e)) => {
                    self.set_value(e.unescape().unwrap());
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:tx" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:tx"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, self.value.get_value());
        write_end_tag(writer, "c:v");

        write_end_tag(writer, "c:tx");
    }
}
