// c:xVal
use super::NumberReference;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::Spreadsheet;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct XValues {
    number_reference: NumberReference,
}
impl XValues {
    pub fn get_number_reference(&self) -> &NumberReference {
        &self.number_reference
    }

    pub fn get_number_reference_mut(&mut self) -> &mut NumberReference {
        &mut self.number_reference
    }

    pub fn set_number_reference(&mut self, value: NumberReference) -> &mut XValues {
        self.number_reference = value;
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
                    b"c:numRef" => {
                        self.number_reference.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:xVal" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:xVal"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:xVal
        write_start_tag(writer, "c:xVal", vec![], false);

        // c:numRef
        self.number_reference.write_to(writer, spreadsheet);

        write_end_tag(writer, "c:xVal");
    }
}
