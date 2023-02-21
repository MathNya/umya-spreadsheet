use super::ClipboardFormatValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::EnumValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ClipboardFormat {
    value: EnumValue<ClipboardFormatValues>,
}
impl ClipboardFormat {
    pub fn get_value(&self) -> &ClipboardFormatValues {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: ClipboardFormatValues) -> &mut Self {
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
                    self.value.set_value_string(e.unescape().unwrap());
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"x:CF" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:CF"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:CF
        write_start_tag(writer, "x:CF", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:CF");
    }
}
