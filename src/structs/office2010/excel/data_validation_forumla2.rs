// x14:formula2
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use std::vec;
use structs::office::excel::Formula;
use structs::Coordinate;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct DataValidationForumla2 {
    value: Formula,
}
impl DataValidationForumla2 {
    pub fn get_value(&self) -> &Formula {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Formula {
        &mut self.value
    }

    pub fn set_value(&mut self, value: Formula) -> &mut Self {
        self.value = value;
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
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"xm:f" => {
                        let mut obj = Formula::default();
                        obj.set_attributes(reader, e);
                        self.value = obj;
                        return;
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"x14:formula2" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "x14:formula2"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "x14:formula2", vec![], false);
        &self.value.write_to(writer);
        write_end_tag(writer, "x14:formula2");
    }
}
