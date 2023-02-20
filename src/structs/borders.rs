// border
use super::BooleanValue;
use super::Border;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Borders {
    left_border: Border,
    right_border: Border,
    top_border: Border,
    bottom_border: Border,
    diagonal_border: Border,
    vertical_border: Border,
    horizontal_border: Border,
    diagonal_down: BooleanValue,
    diagonal_up: BooleanValue,
}
impl Borders {
    // Border style
    pub const BORDER_NONE: &'static str = "none";
    pub const BORDER_DASHDOT: &'static str = "dashDot";
    pub const BORDER_DASHDOTDOT: &'static str = "dashDotDot";
    pub const BORDER_DASHED: &'static str = "dashed";
    pub const BORDER_DOTTED: &'static str = "dotted";
    pub const BORDER_DOUBLE: &'static str = "double";
    pub const BORDER_HAIR: &'static str = "hair";
    pub const BORDER_MEDIUM: &'static str = "medium";
    pub const BORDER_MEDIUMDASHDOT: &'static str = "mediumDashDot";
    pub const BORDER_MEDIUMDASHDOTDOT: &'static str = "mediumDashDotDot";
    pub const BORDER_MEDIUMDASHED: &'static str = "mediumDashed";
    pub const BORDER_SLANTDASHDOT: &'static str = "slantDashDot";
    pub const BORDER_THICK: &'static str = "thick";
    pub const BORDER_THIN: &'static str = "thin";

    pub fn get_left_border(&self) -> &Border {
        &self.left_border
    }

    pub fn get_left_border_mut(&mut self) -> &mut Border {
        &mut self.left_border
    }

    pub fn set_left_border(&mut self, value: Border) -> &mut Self {
        self.left_border = value;
        self
    }

    pub fn get_left(&self) -> &Border {
        &self.left_border
    }

    pub fn get_left_mut(&mut self) -> &mut Border {
        &mut self.left_border
    }

    pub fn set_left(&mut self, value: Border) -> &mut Self {
        self.left_border = value;
        self
    }

    pub fn get_right_border(&self) -> &Border {
        &self.right_border
    }

    pub fn get_right_border_mut(&mut self) -> &mut Border {
        &mut self.right_border
    }

    pub fn set_right_border(&mut self, value: Border) -> &mut Self {
        self.right_border = value;
        self
    }

    pub fn get_right(&self) -> &Border {
        &self.right_border
    }

    pub fn get_right_mut(&mut self) -> &mut Border {
        &mut self.right_border
    }

    pub fn set_right(&mut self, value: Border) -> &mut Self {
        self.right_border = value;
        self
    }

    pub fn get_top_border(&self) -> &Border {
        &self.top_border
    }

    pub fn get_top_border_mut(&mut self) -> &mut Border {
        &mut self.top_border
    }

    pub fn set_top_border(&mut self, value: Border) -> &mut Self {
        self.top_border = value;
        self
    }

    pub fn get_top(&self) -> &Border {
        &self.top_border
    }

    pub fn get_top_mut(&mut self) -> &mut Border {
        &mut self.top_border
    }

    pub fn set_top(&mut self, value: Border) -> &mut Self {
        self.top_border = value;
        self
    }

    pub fn get_bottom_border(&self) -> &Border {
        &self.bottom_border
    }

    pub fn get_bottom_border_mut(&mut self) -> &mut Border {
        &mut self.bottom_border
    }

    pub fn set_bottom_border(&mut self, value: Border) -> &mut Self {
        self.bottom_border = value;
        self
    }

    pub fn get_bottom(&self) -> &Border {
        &self.bottom_border
    }

    pub fn get_bottom_mut(&mut self) -> &mut Border {
        &mut self.bottom_border
    }

    pub fn set_bottom(&mut self, value: Border) -> &mut Self {
        self.bottom_border = value;
        self
    }

    pub fn get_diagonal_border(&self) -> &Border {
        &self.diagonal_border
    }

    pub fn get_diagonal_border_mut(&mut self) -> &mut Border {
        &mut self.diagonal_border
    }

    pub fn set_diagonal_border(&mut self, value: Border) -> &mut Self {
        self.diagonal_border = value;
        self
    }

    pub fn get_diagonal(&self) -> &Border {
        &self.diagonal_border
    }

    pub fn get_diagonal_mut(&mut self) -> &mut Border {
        &mut self.diagonal_border
    }

    pub fn set_diagonal(&mut self, value: Border) -> &mut Self {
        self.diagonal_border = value;
        self
    }

    pub fn get_vertical_border(&self) -> &Border {
        &self.vertical_border
    }

    pub fn get_vertical_border_mut(&mut self) -> &mut Border {
        &mut self.vertical_border
    }

    pub fn set_vertical_border(&mut self, value: Border) -> &mut Self {
        self.vertical_border = value;
        self
    }

    pub fn get_horizontal_border(&self) -> &Border {
        &self.horizontal_border
    }

    pub fn get_horizontal_border_mut(&mut self) -> &mut Border {
        &mut self.horizontal_border
    }

    pub fn set_horizontal_border(&mut self, value: Border) -> &mut Self {
        self.horizontal_border = value;
        self
    }

    pub fn get_diagonal_down(&self) -> &bool {
        self.diagonal_down.get_value()
    }

    pub fn set_diagonal_down(&mut self, value: bool) {
        self.diagonal_down.set_value(value);
    }

    pub fn get_diagonal_up(&self) -> &bool {
        self.diagonal_up.get_value()
    }

    pub fn set_diagonal_up(&mut self, value: bool) {
        self.diagonal_up.set_value(value);
    }

    pub(crate) fn get_defalut_value() -> Self {
        Self::default()
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}{}{}{}{}{}",
                &self.get_left_border().get_hash_code(),
                &self.get_right_border().get_hash_code(),
                &self.get_top_border().get_hash_code(),
                &self.get_bottom_border().get_hash_code(),
                &self.get_diagonal_border().get_hash_code(),
                &self.get_vertical_border().get_hash_code(),
                &self.get_horizontal_border().get_hash_code(),
                &self.diagonal_down.get_value_string(),
                &self.diagonal_up.get_value_string()
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"diagonalUp") {
            Some(v) => {
                self.diagonal_up.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"diagonalDown") {
            Some(v) => {
                self.diagonal_down.set_value_string(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"left" => {
                        self.left_border.set_attributes(reader, e);
                    }
                    b"right" => {
                        self.right_border.set_attributes(reader, e);
                    }
                    b"top" => {
                        self.top_border.set_attributes(reader, e);
                    }
                    b"bottom" => {
                        self.bottom_border.set_attributes(reader, e);
                    }
                    b"diagonal" => {
                        self.diagonal_border.set_attributes(reader, e);
                    }
                    b"vertical" => {
                        self.vertical_border.set_attributes(reader, e);
                    }
                    b"horizontal" => {
                        self.horizontal_border.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"border" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "border"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // border
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.diagonal_up.has_value() {
            attributes.push(("diagonalUp", self.diagonal_up.get_value_string()));
        }
        if self.diagonal_down.has_value() {
            attributes.push(("diagonalDown", self.diagonal_down.get_value_string()));
        }
        write_start_tag(writer, "border", attributes, false);

        // left
        self.left_border.write_to_left(writer);

        // right
        self.right_border.write_to_right(writer);

        // top
        self.top_border.write_to_top(writer);

        // bottom
        self.bottom_border.write_to_bottom(writer);

        // diagonal
        self.diagonal_border.write_to_diagonal(writer);

        // vertical
        if self.vertical_border.get_hash_code() != Border::default().get_hash_code() {
            self.vertical_border.write_to_vertical(writer);
        }

        // horizontal
        if self.horizontal_border.get_hash_code() != Border::default().get_hash_code() {
            self.horizontal_border.write_to_horizontal(writer);
        }

        write_end_tag(writer, "border");
    }
}
