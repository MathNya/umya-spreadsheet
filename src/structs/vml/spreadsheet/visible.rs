use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::TrueFalseBlankValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Visible {
    value: TrueFalseBlankValue,
}
impl Visible {
    pub fn get_value(&self) -> &Option<bool> {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    self.value.set_value_string(e.unescape().unwrap());
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"x:Visible" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:Visible"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Visible
        if self.value.has_value() {
            write_start_tag(writer, "x:Visible", vec![], false);
            write_text_node(writer, self.value.get_value_string2());
            write_end_tag(writer, "x:Visible");
        } else {
            write_start_tag(writer, "x:Visible", vec![], true);
        }
    }
}
