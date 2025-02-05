use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use crate::{
    helper::coordinate::{
        adjustment_insert_coordinate, adjustment_remove_coordinate, is_remove_coordinate,
    },
    reader::driver::xml_read_loop,
    traits::AdjustmentCoordinate,
    writer::driver::{write_end_tag, write_start_tag, write_text_node},
};

#[derive(Clone, Default, Debug)]
pub struct Anchor {
    left_column: u32,
    left_offset: u32,
    top_row: u32,
    top_offset: u32,
    right_column: u32,
    right_offset: u32,
    bottom_row: u32,
    bottom_offset: u32,
}

impl Anchor {
    #[inline]
    #[must_use]
    pub fn left_column(&self) -> u32 {
        self.left_column
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use left_column()")]
    pub fn get_left_column(&self) -> u32 {
        self.left_column()
    }

    #[inline]
    pub fn set_left_column(&mut self, value: u32) -> &mut Self {
        self.left_column = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn left_offset(&self) -> u32 {
        self.left_offset
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use left_offset()")]
    pub fn get_left_offset(&self) -> u32 {
        self.left_offset()
    }

    #[inline]
    pub fn set_left_offset(&mut self, value: u32) -> &mut Self {
        self.left_offset = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn top_row(&self) -> u32 {
        self.top_row
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use top_row()")]
    pub fn get_top_row(&self) -> u32 {
        self.top_row()
    }

    #[inline]
    pub fn set_top_row(&mut self, value: u32) -> &mut Self {
        self.top_row = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn top_offset(&self) -> u32 {
        self.top_offset
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use top_offset()")]
    pub fn get_top_offset(&self) -> u32 {
        self.top_offset()
    }

    #[inline]
    pub fn set_top_offset(&mut self, value: u32) -> &mut Self {
        self.top_offset = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn right_column(&self) -> u32 {
        self.right_column
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use right_column()")]
    pub fn get_right_column(&self) -> u32 {
        self.right_column()
    }

    #[inline]
    pub fn set_right_column(&mut self, value: u32) -> &mut Self {
        self.right_column = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn right_offset(&self) -> u32 {
        self.right_offset
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use right_offset()")]
    pub fn get_right_offset(&self) -> u32 {
        self.right_offset()
    }

    #[inline]
    pub fn set_right_offset(&mut self, value: u32) -> &mut Self {
        self.right_offset = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn bottom_row(&self) -> u32 {
        self.bottom_row
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bottom_row()")]
    pub fn get_bottom_row(&self) -> u32 {
        self.bottom_row()
    }

    #[inline]
    pub fn set_bottom_row(&mut self, value: u32) -> &mut Self {
        self.bottom_row = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn bottom_offset(&self) -> u32 {
        self.bottom_offset
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use bottom_offset()")]
    pub fn get_bottom_offset(&self) -> u32 {
        self.bottom_offset()
    }

    #[inline]
    pub fn set_bottom_offset(&mut self, value: u32) -> &mut Self {
        self.bottom_offset = value;
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn adjustment_insert_row(&mut self, num_rows: u32) {
        self.top_row += num_rows;
        self.bottom_row += num_rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn adjustment_insert_column(&mut self, num_cols: u32) {
        self.left_column += num_cols;
        self.right_column += num_cols;
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn adjustment_remove_row(&mut self, num_rows: u32) {
        self.top_row = if self.top_row > num_rows {
            self.top_row - num_rows
        } else {
            1
        };
        self.bottom_row = if self.bottom_row > num_rows {
            self.bottom_row - num_rows
        } else {
            1
        };
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn adjustment_remove_column(&mut self, num_cols: u32) {
        self.left_column = if self.left_column > num_cols {
            self.left_column - num_cols
        } else {
            1
        };
        self.right_column = if self.right_column > num_cols {
            self.right_column - num_cols
        } else {
            1
        };
    }

    #[inline]
    fn number(value: Option<&&str>) -> u32 {
        match value {
            Some(v) => (*v).to_string().trim().parse::<u32>().unwrap_or(0),
            None => 0,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use number()")]
    fn get_number(value: Option<&&str>) -> u32 {
        Self::number(value)
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
                let text = e.unescape().unwrap();
                let split_str: Vec<&str> = text.split(',').collect();
                self.set_left_column(Self::number(split_str.first()));
                self.set_left_offset(Self::number(split_str.get(1)));
                self.set_top_row(Self::number(split_str.get(2)));
                self.set_top_offset(Self::number(split_str.get(3)));
                self.set_right_column(Self::number(split_str.get(4)));
                self.set_right_offset(Self::number(split_str.get(5)));
                self.set_bottom_row(Self::number(split_str.get(6)));
                self.set_bottom_offset(Self::number(split_str.get(7)));
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:Anchor" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:Anchor")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Anchor
        let anchor = format!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.left_column(),
            self.left_offset(),
            self.top_row(),
            self.top_offset(),
            self.right_column(),
            self.right_offset(),
            self.bottom_row(),
            self.bottom_offset()
        );
        write_start_tag(writer, "x:Anchor", vec![], false);
        write_text_node(writer, &anchor);
        write_end_tag(writer, "x:Anchor");
    }
}
impl AdjustmentCoordinate for Anchor {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.left_column =
            adjustment_insert_coordinate(self.left_column + 1, root_col_num, offset_col_num) - 1;
        self.right_column =
            adjustment_insert_coordinate(self.right_column + 1, root_col_num, offset_col_num) - 1;

        self.top_row =
            adjustment_insert_coordinate(self.top_row + 1, root_row_num, offset_row_num) - 1;
        self.bottom_row =
            adjustment_insert_coordinate(self.bottom_row + 1, root_row_num, offset_row_num) - 1;
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.left_column =
            adjustment_remove_coordinate(self.left_column + 1, root_col_num, offset_col_num) - 1;
        self.right_column =
            adjustment_remove_coordinate(self.right_column + 1, root_col_num, offset_col_num) - 1;

        self.top_row =
            adjustment_remove_coordinate(self.top_row + 1, root_row_num, offset_row_num) - 1;
        self.bottom_row =
            adjustment_remove_coordinate(self.bottom_row + 1, root_row_num, offset_row_num) - 1;
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        is_remove_coordinate(self.left_column + 1, root_col_num, offset_col_num)
            || is_remove_coordinate(self.right_column + 1, root_col_num, offset_col_num)
            || is_remove_coordinate(self.top_row + 1, root_row_num, offset_row_num)
            || is_remove_coordinate(self.bottom_row + 1, root_row_num, offset_row_num)
    }
}
