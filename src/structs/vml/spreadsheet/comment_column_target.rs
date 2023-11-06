use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct CommentColumnTarget {
    value: UInt32Value,
}

impl CommentColumnTarget {
    pub fn get_value(&self) -> &u32 {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn adjustment_insert_column(&mut self, num_cols: &u32) {
        let value = self.value.get_value() + num_cols;
        self.value.set_value(value);
    }

    pub(crate) fn adjustment_remove_column(&mut self, num_cols: &u32) {
        if self.value.get_value() > num_cols {
            let value = self.value.get_value() - num_cols;
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
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.value.set_value_string(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:Column" {
                    return
                }
            },
            Event::Eof => panic!("Error not find {} end element", "x:Column")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Column
        write_start_tag(writer, "x:Column", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:Column");
    }
}
