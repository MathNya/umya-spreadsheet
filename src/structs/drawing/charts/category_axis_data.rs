// c:cat
use super::StringReference;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct CategoryAxisData {
    string_reference: StringReference,
}
impl CategoryAxisData {
    pub fn get_string_reference(&self)-> &StringReference {
        &self.string_reference
    }

    pub fn get_string_reference_mut(&mut self)-> &mut StringReference {
        &mut self.string_reference
    }

    pub fn set_string_reference(&mut self, value:StringReference)-> &mut CategoryAxisData {
        self.string_reference = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:strRef" => {
                            self.string_reference.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:cat" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:cat"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:cat
        write_start_tag(writer, "c:cat", vec![], false);

        // c:strRef
        &self.string_reference.write_to(writer);

        write_end_tag(writer, "c:cat");
    }
}
