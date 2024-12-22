// x14:formula2
use std::io::Cursor;
use std::vec;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use crate::structs::office::excel::Formula;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Default, Debug, Clone)]
pub struct DataValidationForumla2 {
    value: Formula,
}
impl DataValidationForumla2 {
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> &Formula {
        &self.value
    }

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut Formula {
        &mut self.value
    }

    #[inline]
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
                Ok(Event::Start(ref e)) =>
                    if e.name().into_inner() == b"xm:f" {
                        let mut obj = Formula::default();
                        obj.set_attributes(reader, e);
                        self.value = obj;
                        return;
                    },
                Ok(Event::End(ref e)) =>
                    if e.name().into_inner() == b"x14:formula2" {
                        return;
                    },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "x14:formula2"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "x14:formula2", vec![], false);
        self.value.write_to(writer);
        write_end_tag(writer, "x14:formula2");
    }
}
