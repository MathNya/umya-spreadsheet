use super::run_properties::RunProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Run {
    text: String,
    run_properties: RunProperties,
}
impl Run {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) {
        self.text = value.into();
    }

    pub fn get_run_properties(&self) -> &RunProperties {
        &self.run_properties
    }

    pub fn get_run_properties_mut(&mut self) -> &mut RunProperties {
        &mut self.run_properties
    }

    pub fn set_run_properties(&mut self, value: RunProperties) {
        self.run_properties = value;
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
                    b"a:rPr" => {
                        self.run_properties.set_attributes(reader, e, false);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"a:rPr" => {
                        self.run_properties.set_attributes(reader, e, true);
                    }
                    _ => (),
                },
                Ok(Event::Text(e)) => {
                    self.set_text(e.unescape().unwrap());
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"a:r" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:r"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:r
        write_start_tag(writer, "a:r", vec![], false);

        // a:rPr
        self.run_properties.write_to_rpr(writer);

        // a:t
        write_start_tag(writer, "a:t", vec![], false);
        write_text_node(writer, &self.text);
        write_end_tag(writer, "a:t");

        write_end_tag(writer, "a:r");
    }
}
