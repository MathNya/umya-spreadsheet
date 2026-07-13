use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    helper::coordinate::{
        adjustment_insert_coordinate,
        adjustment_remove_coordinate,
        is_remove_coordinate,
    },
    structs::UInt32Value,
    traits::AdjustmentValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct CommentColumnTarget {
    value: UInt32Value,
}

impl CommentColumnTarget {
    #[inline]
    #[must_use]
    pub fn value(&self) -> u32 {
        self.value.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value(&self) -> u32 {
        self.value()
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
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.value
            .set_value_string(crate::helper::utils::unescape_xml_text(&text));
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Column
        write_start_tag(writer, "x:Column", vec![], false);
        write_text_node(writer, self.value.value_string());
        write_end_tag(writer, "x:Column");
    }
}
impl AdjustmentValue for CommentColumnTarget {
    #[inline]
    fn adjustment_insert_value(&mut self, root_num: u32, offset_num: u32) {
        self.value.set_value(
            adjustment_insert_coordinate(self.value.value() + 1, root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn adjustment_remove_value(&mut self, root_num: u32, offset_num: u32) {
        self.value.set_value(
            adjustment_remove_coordinate(self.value.value() + 1, root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn is_remove_value(&self, root_num: u32, offset_num: u32) -> bool {
        is_remove_coordinate(self.value.value() + 1, root_num, offset_num)
    }
}
