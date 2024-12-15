use crate::helper::coordinate::*;
use crate::reader::driver::*;
use crate::structs::UInt32Value;
use crate::traits::AdjustmentValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CommentColumnTarget {
    value: UInt32Value,
}

impl CommentColumnTarget {
    #[inline]
    pub fn get_value(&self) -> &u32 {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
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
            Event::Eof => panic!("Error: Could not find {} end element", "x:Column")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Column
        write_start_tag(writer, "x:Column", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:Column");
    }
}
impl AdjustmentValue for CommentColumnTarget {
    #[inline]
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_insert_coordinate(&(self.value.get_value() + 1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_remove_coordinate(&(self.value.get_value() + 1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn is_remove_value(&self, root_num: &u32, offset_num: &u32) -> bool {
        is_remove_coordinate(&(self.value.get_value() + 1), root_num, offset_num)
    }
}
