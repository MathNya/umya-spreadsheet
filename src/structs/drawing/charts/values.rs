use std::io::Cursor;

use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};

// c:val
use super::NumberReference;
use crate::{
    structs::Workbook,
    writer::driver::{write_end_tag, write_start_tag},
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct Values {
    number_reference: NumberReference,
}

impl Values {
    #[must_use]
    pub fn get_number_reference(&self) -> &NumberReference {
        &self.number_reference
    }

    pub fn get_number_reference_mut(&mut self) -> &mut NumberReference {
        &mut self.number_reference
    }

    pub fn set_number_reference(&mut self, value: NumberReference) -> &mut Values {
        self.number_reference = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().0 == b"c:numRef" {
                    self.number_reference.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:val" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:val"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:val
        write_start_tag(writer, "c:val", vec![], false);

        // c:numRef
        self.number_reference.write_to(writer, wb);

        write_end_tag(writer, "c:val");
    }
}
