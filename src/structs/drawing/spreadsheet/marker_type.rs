// xdr:from,xdr:to
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    helper::coordinate::{
        adjustment_insert_coordinate,
        adjustment_remove_coordinate,
        coordinate_from_index,
        index_from_coordinate,
        is_remove_coordinate,
    },
    traits::AdjustmentCoordinate,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct MarkerType {
    col:     u32,
    col_off: i32,
    row:     u32,
    row_off: i32,
}
impl MarkerType {
    #[inline]
    #[must_use]
    pub fn col(&self) -> u32 {
        self.col
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use col()")]
    pub fn get_col(&self) -> u32 {
        self.col()
    }

    #[inline]
    pub fn set_col(&mut self, value: u32) -> &mut Self {
        self.col = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn col_off(&self) -> i32 {
        self.col_off
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use col_off()")]
    pub fn get_col_off(&self) -> i32 {
        self.col_off()
    }

    #[inline]
    pub fn set_col_off(&mut self, value: i32) -> &mut Self {
        self.col_off = value;
        self
    }

    #[inline]
    pub fn add_col_off(&mut self, value: i32) -> &mut Self {
        self.col_off += value;
        self
    }

    #[inline]
    #[must_use]
    pub fn row(&self) -> u32 {
        self.row
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row()")]
    pub fn get_row(&self) -> u32 {
        self.row()
    }

    #[inline]
    pub fn set_row(&mut self, value: u32) -> &mut Self {
        self.row = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn row_off(&self) -> i32 {
        self.row_off
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_off()")]
    pub fn get_row_off(&self) -> i32 {
        self.row_off()
    }

    #[inline]
    pub fn set_row_off(&mut self, value: i32) -> &mut Self {
        self.row_off = value;
        self
    }

    #[inline]
    pub fn add_row_off(&mut self, value: i32) -> &mut Self {
        self.row_off += value;
        self
    }

    #[inline]
    #[must_use]
    pub fn coordinate(&self) -> String {
        coordinate_from_index(self.col + 1, self.row + 1)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate()")]
    pub fn get_coordinate(&self) -> String {
        self.coordinate()
    }

    #[inline]
    pub fn set_coordinate<S: Into<String>>(&mut self, value: S) {
        let (col, row, ..) = index_from_coordinate(value.into());
        self.col = col.unwrap() - 1;
        self.row = row.unwrap() - 1;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut string_value: String = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:col" => {
                        self.col = string_value.parse::<u32>().unwrap();
                    }
                    b"xdr:colOff" => {
                        self.col_off = string_value.parse::<i32>().unwrap();
                    }
                    b"xdr:row" => {
                        self.row = string_value.parse::<u32>().unwrap();
                    }
                    b"xdr:rowOff" => {
                        self.row_off = string_value.parse::<i32>().unwrap();
                    }
                    b"xdr:from" | b"xdr:to" => return,
                    _ => (),
                },
                Ok(Event::Eof) => {
                    panic!("Error: Could not find {} end element", "xdr:from,xdr:to")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    #[inline]
    pub(crate) fn write_to_from(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "xdr:from");
    }

    #[inline]
    pub(crate) fn write_to_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "xdr:to");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // xdr:to
        write_start_tag(writer, tag_name, vec![], false);

        // xdr:col
        write_start_tag(writer, "xdr:col", vec![], false);
        write_text_node(writer, self.col.to_string());
        write_end_tag(writer, "xdr:col");

        // xdr:colOff
        write_start_tag(writer, "xdr:colOff", vec![], false);
        write_text_node(writer, self.col_off.to_string());
        write_end_tag(writer, "xdr:colOff");

        // xdr:row
        write_start_tag(writer, "xdr:row", vec![], false);
        write_text_node(writer, self.row.to_string());
        write_end_tag(writer, "xdr:row");

        // xdr:rowOff
        write_start_tag(writer, "xdr:rowOff", vec![], false);
        write_text_node(writer, self.row_off.to_string());
        write_end_tag(writer, "xdr:rowOff");

        write_end_tag(writer, tag_name);
    }
}
impl AdjustmentCoordinate for MarkerType {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.col = adjustment_insert_coordinate(self.col + 1, root_col_num, offset_col_num) - 1;
        self.row = adjustment_insert_coordinate(self.row + 1, root_row_num, offset_row_num) - 1;
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.col = adjustment_remove_coordinate(self.col + 1, root_col_num, offset_col_num) - 1;
        self.row = adjustment_remove_coordinate(self.row + 1, root_row_num, offset_row_num) - 1;
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        is_remove_coordinate(self.col + 1, root_col_num, offset_col_num)
            || is_remove_coordinate(self.row + 1, root_row_num, offset_row_num)
    }
}
