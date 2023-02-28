use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

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
    pub fn get_left_column(&self) -> &u32 {
        &self.left_column
    }

    pub fn set_left_column(&mut self, value: u32) {
        self.left_column = value;
    }

    pub fn get_left_offset(&self) -> &u32 {
        &self.left_offset
    }

    pub fn set_left_offset(&mut self, value: u32) {
        self.left_offset = value;
    }

    pub fn get_top_row(&self) -> &u32 {
        &self.top_row
    }

    pub fn set_top_row(&mut self, value: u32) {
        self.top_row = value;
    }

    pub fn get_top_offset(&self) -> &u32 {
        &self.top_offset
    }

    pub fn set_top_offset(&mut self, value: u32) {
        self.top_offset = value;
    }

    pub fn get_right_column(&self) -> &u32 {
        &self.right_column
    }

    pub fn set_right_column(&mut self, value: u32) {
        self.right_column = value;
    }

    pub fn get_right_offset(&self) -> &u32 {
        &self.right_offset
    }

    pub fn set_right_offset(&mut self, value: u32) {
        self.right_offset = value;
    }

    pub fn get_bottom_row(&self) -> &u32 {
        &self.bottom_row
    }

    pub fn set_bottom_row(&mut self, value: u32) {
        self.bottom_row = value;
    }

    pub fn get_bottom_offset(&self) -> &u32 {
        &self.bottom_offset
    }

    pub fn set_bottom_offset(&mut self, value: u32) {
        self.bottom_offset = value;
    }

    pub(crate) fn adjustment_insert_row(&mut self, num_rows: &u32) {
        self.top_row += num_rows;
        self.bottom_row += num_rows;
    }

    pub(crate) fn adjustment_insert_column(&mut self, num_cols: &u32) {
        self.left_column += num_cols;
        self.right_column += num_cols;
    }

    pub(crate) fn adjustment_remove_row(&mut self, num_rows: &u32) {
        self.top_row = if &self.top_row > num_rows {
            self.top_row - num_rows
        } else {
            1
        };
        self.bottom_row = if &self.bottom_row > num_rows {
            self.bottom_row - num_rows
        } else {
            1
        };
    }

    pub(crate) fn adjustment_remove_column(&mut self, num_cols: &u32) {
        self.left_column = if &self.left_column > num_cols {
            self.left_column - num_cols
        } else {
            1
        };
        self.right_column = if &self.right_column > num_cols {
            self.right_column - num_cols
        } else {
            1
        };
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
                    let text = e.unescape().unwrap();
                    let split_str: Vec<&str> = text.split(", ").collect();
                    self.set_left_column(Self::get_number(split_str.first()));
                    self.set_left_offset(Self::get_number(split_str.get(1)));
                    self.set_top_row(Self::get_number(split_str.get(2)));
                    self.set_top_offset(Self::get_number(split_str.get(3)));
                    self.set_right_column(Self::get_number(split_str.get(4)));
                    self.set_right_offset(Self::get_number(split_str.get(5)));
                    self.set_bottom_row(Self::get_number(split_str.get(6)));
                    self.set_bottom_offset(Self::get_number(split_str.get(7)));
                }
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"x:Anchor" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:Anchor"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    fn get_number(value: Option<&&str>) -> u32 {
        match value {
            Some(v) => v.to_string().parse::<u32>().unwrap_or(0),
            None => 0,
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Anchor
        let anchor = format!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.get_left_column(),
            self.get_left_offset(),
            self.get_top_row(),
            self.get_top_offset(),
            self.get_right_column(),
            self.get_right_offset(),
            self.get_bottom_row(),
            self.get_bottom_offset()
        );
        write_start_tag(writer, "x:Anchor", vec![], false);
        write_text_node(writer, &anchor);
        write_end_tag(writer, "x:Anchor");
    }
}
