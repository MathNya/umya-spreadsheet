use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CommentRowTarget {
    value: UInt32Value,
}
impl CommentRowTarget {
    pub fn get_value(&self) -> &u32 {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn adjustment_insert_row(&mut self, num_rows: &u32) {
        let value = self.value.get_value() + num_rows;
        self.value.set_value(value);
    }

    pub(crate) fn adjustment_remove_row(&mut self, num_row: &u32) {
        if self.value.get_value() > num_row {
            let value = self.value.get_value() - num_row;
            self.value.set_value(value);
        } else {
            self.value.set_value(1);
        }
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
                    b"x:Row" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:Row"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Row
        write_start_tag(writer, "x:Row", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:Row");
    }
}
