// border
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    BooleanValue,
    Border,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

pub(crate) enum BordersIndex {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    Diagonal = 4,
    Vertical = 5,
    Horizontal = 6,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Borders {
    data:          Box<[Border; 7]>,
    diagonal_down: BooleanValue,
    diagonal_up:   BooleanValue,
}

macro_rules! border_const {
    ($($name:ident => $value:expr),*) => {
        impl Borders {
            $(pub const $name: &'static str = $value;)*
        }
    }
}

border_const! {
    BORDER_DASHDOT => "dashDot",
    BORDER_DASHDOTDOT => "dashDotDot", 
    BORDER_DASHED => "dashed",
    BORDER_DOTTED => "dotted",
    BORDER_DOUBLE => "double",
    BORDER_HAIR => "hair",
    BORDER_MEDIUM => "medium",
    BORDER_MEDIUMDASHDOT => "mediumDashDot",
    BORDER_MEDIUMDASHDOTDOT => "mediumDashDotDot",
    BORDER_MEDIUMDASHED => "mediumDashed",
    BORDER_NONE => "none",
    BORDER_SLANTDASHDOT => "slantDashDot",
    BORDER_THICK => "thick",
    BORDER_THIN => "thin"
}

impl Borders {

    #[inline]
    pub fn left(&self) -> &Border {
        &self.data[BordersIndex::Left as usize]
    }

    #[inline]
    pub fn left_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Left as usize]
    }

    #[inline]
    pub fn left_border(&self) -> &Border {
        self.left()
    }

    #[inline]
    pub fn left_border_mut(&mut self) -> &mut Border {
        self.left_mut()
    }

    #[inline]
    pub fn set_left(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Left as usize] = value;
        self
    }

    #[inline]
    pub fn set_left_border(&mut self, value: Border) -> &mut Self {
        self.set_left(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use left()")]
    pub fn get_left(&self) -> &Border {
        self.left()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use left_mut()")]
    pub fn get_left_mut(&mut self) -> &mut Border {
        self.left_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use left_border()")]
    pub fn get_left_border(&self) -> &Border {
        self.left_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use left_border_mut()")]
    pub fn get_left_border_mut(&mut self) -> &mut Border {
        self.left_border_mut()
    }

    #[inline]
    pub fn right(&self) -> &Border {
        &self.data[BordersIndex::Right as usize]
    }

    #[inline]
    pub fn right_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Right as usize]
    }

    #[inline]
    pub fn right_border(&self) -> &Border {
        self.right()
    }

    #[inline]
    pub fn right_border_mut(&mut self) -> &mut Border {
        self.right_mut()
    }

    #[inline]
    pub fn set_right(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Right as usize] = value;
        self
    }

    #[inline]
    pub fn set_right_border(&mut self, value: Border) -> &mut Self {
        self.set_right(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use right()")]
    pub fn get_right(&self) -> &Border {
        self.right()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use right_mut()")]
    pub fn get_right_mut(&mut self) -> &mut Border {
        self.right_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use right_border()")]
    pub fn get_right_border(&self) -> &Border {
        self.right_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use right_border_mut()")]
    pub fn get_right_border_mut(&mut self) -> &mut Border {
        self.right_border_mut()
    }

    #[inline]
    pub fn top(&self) -> &Border {
        &self.data[BordersIndex::Top as usize]
    }

    #[inline]
    pub fn top_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Top as usize]
    }

    #[inline]
    pub fn top_border(&self) -> &Border {
        self.top()
    }

    #[inline]
    pub fn top_border_mut(&mut self) -> &mut Border {
        self.top_mut()
    }

    #[inline]
    pub fn set_top(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Top as usize] = value;
        self
    }

    #[inline]
    pub fn set_top_border(&mut self, value: Border) -> &mut Self {
        self.set_top(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use top()")]
    pub fn get_top(&self) -> &Border {
        self.top()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use top_mut()")]
    pub fn get_top_mut(&mut self) -> &mut Border {
        self.top_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use top_border()")]
    pub fn get_top_border(&self) -> &Border {
        self.top_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use top_border_mut()")]
    pub fn get_top_border_mut(&mut self) -> &mut Border {
        self.top_border_mut()
    }

    #[inline]
    pub fn bottom(&self) -> &Border {
        &self.data[BordersIndex::Bottom as usize]
    }

    #[inline]
    pub fn bottom_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Bottom as usize]
    }

    #[inline]
    pub fn bottom_border(&self) -> &Border {
        self.bottom()
    }

    #[inline]
    pub fn bottom_border_mut(&mut self) -> &mut Border {
        self.bottom_mut()
    }

    #[inline]
    pub fn set_bottom(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Bottom as usize] = value;
        self
    }

    #[inline]
    pub fn set_bottom_border(&mut self, value: Border) -> &mut Self {
        self.set_bottom(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use bottom()")]
    pub fn get_bottom(&self) -> &Border {
        self.bottom()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use bottom_mut()")]
    pub fn get_bottom_mut(&mut self) -> &mut Border {
        self.bottom_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use bottom_border()")]
    pub fn get_bottom_border(&self) -> &Border {
        self.bottom_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use bottom_border_mut()")]
    pub fn get_bottom_border_mut(&mut self) -> &mut Border {
        self.bottom_border_mut()
    }

    #[inline]
    pub fn diagonal(&self) -> &Border {
        &self.data[BordersIndex::Diagonal as usize]
    }

    #[inline]
    pub fn diagonal_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Diagonal as usize]
    }

    #[inline]
    pub fn diagonal_border(&self) -> &Border {
        self.diagonal()
    }

    #[inline]
    pub fn diagonal_border_mut(&mut self) -> &mut Border {
        self.diagonal_mut()
    }

    #[inline]
    pub fn set_diagonal(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Diagonal as usize] = value;
        self
    }

    #[inline]
    pub fn set_diagonal_border(&mut self, value: Border) -> &mut Self {
        self.set_diagonal(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use diagonal()")]
    pub fn get_diagonal(&self) -> &Border {
        self.diagonal()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use diagonal_mut()")]
    pub fn get_diagonal_mut(&mut self) -> &mut Border {
        self.diagonal_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use diagonal_border()")]
    pub fn get_diagonal_border(&self) -> &Border {
        self.diagonal_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use diagonal_border_mut()")]
    pub fn get_diagonal_border_mut(&mut self) -> &mut Border {
        self.diagonal_border_mut()
    }

    #[inline]
    pub fn vertical(&self) -> &Border {
        &self.data[BordersIndex::Vertical as usize]
    }

    #[inline]
    pub fn vertical_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Vertical as usize]
    }

    #[inline]
    pub fn vertical_border(&self) -> &Border {
        self.vertical()
    }

    #[inline]
    pub fn vertical_border_mut(&mut self) -> &mut Border {
        self.vertical_mut()
    }

    #[inline]
    pub fn set_vertical(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Vertical as usize] = value;
        self
    }

    #[inline]
    pub fn set_vertical_border(&mut self, value: Border) -> &mut Self {
        self.set_vertical(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use vertical()")]
    pub fn get_vertical(&self) -> &Border {
        self.vertical()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use vertical_mut()")]
    pub fn get_vertical_mut(&mut self) -> &mut Border {
        self.vertical_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use vertical_border()")]
    pub fn get_vertical_border(&self) -> &Border {
        self.vertical_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use vertical_border_mut()")]
    pub fn get_vertical_border_mut(&mut self) -> &mut Border {
        self.vertical_border_mut()
    }

    #[inline]
    pub fn horizontal(&self) -> &Border {
        &self.data[BordersIndex::Horizontal as usize]
    }

    #[inline]
    pub fn horizontal_mut(&mut self) -> &mut Border {
        &mut self.data[BordersIndex::Horizontal as usize]
    }

    #[inline]
    pub fn horizontal_border(&self) -> &Border {
        self.horizontal()
    }

    #[inline]
    pub fn horizontal_border_mut(&mut self) -> &mut Border {
        self.horizontal_mut()
    }

    #[inline]
    pub fn set_horizontal(&mut self, value: Border) -> &mut Self {
        self.data[BordersIndex::Horizontal as usize] = value;
        self
    }

    #[inline]
    pub fn set_horizontal_border(&mut self, value: Border) -> &mut Self {
        self.set_horizontal(value)
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use horizontal()")]
    pub fn get_horizontal(&self) -> &Border {
        self.horizontal()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use horizontal_mut()")]
    pub fn get_horizontal_mut(&mut self) -> &mut Border {
        self.horizontal_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use horizontal_border()")]
    pub fn get_horizontal_border(&self) -> &Border {
        self.horizontal_border()
    }

    #[inline]
    #[deprecated(since = "3.0.1", note = "Use horizontal_border_mut()")]
    pub fn get_horizontal_border_mut(&mut self) -> &mut Border {
        self.horizontal_border_mut()
    }

    #[inline]
    pub fn diagonal_down(&self) -> bool {
        self.diagonal_down.value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use diagonal_down()")]
    pub fn get_diagonal_down(&self) -> bool {
        self.diagonal_down()
    }

    #[inline]
    pub fn set_diagonal_down(&mut self, value: bool) {
        self.diagonal_down.set_value(value);
    }

    #[inline]
    pub fn diagonal_up(&self) -> bool {
        self.diagonal_up.value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use diagonal_up()")]
    pub fn get_diagonal_up(&self) -> bool {
        self.diagonal_up()
    }

    #[inline]
    pub fn set_diagonal_up(&mut self, value: bool) {
        self.diagonal_up.set_value(value);
    }

    #[inline]
    pub(crate) fn default_value() -> Self {
        Self::default()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use default_value()")]
    pub(crate) fn get_default_value() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn hash_code(&self) -> String {
        crate::helper::utils::md5_hash(format!(
            "{}{}{}{}{}{}{}{}{}",
            self.left_border().hash_code(),
            self.right_border().hash_code(),
            self.top_border().hash_code(),
            self.bottom_border().hash_code(),
            self.diagonal_border().hash_code(),
            self.vertical_border().hash_code(),
            self.horizontal_border().hash_code(),
            self.diagonal_down.value_string(),
            self.diagonal_up.value_string()
        ))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    #[inline]
    pub(crate) fn is_visually_empty(&self) -> bool {
        self.data[BordersIndex::Left as usize].is_visually_empty()
            || self.data[BordersIndex::Right as usize].is_visually_empty()
            || self.data[BordersIndex::Top as usize].is_visually_empty()
            || self.data[BordersIndex::Bottom as usize].is_visually_empty()
            || self.data[BordersIndex::Diagonal as usize].is_visually_empty()
            || self.data[BordersIndex::Vertical as usize].is_visually_empty()
            || self.data[BordersIndex::Horizontal as usize].is_visually_empty()
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, diagonal_up, "diagonalUp");
        set_string_from_xml!(self, e, diagonal_down, "diagonalDown");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"left" => {
                        self.data[BordersIndex::Left as usize].set_attributes(reader, e, true);
                    }
                    b"right" => {
                        self.data[BordersIndex::Right as usize].set_attributes(reader, e, true);
                    }
                    b"top" => {
                        self.data[BordersIndex::Top as usize].set_attributes(reader, e, true);
                    }
                    b"bottom" => {
                        self.data[BordersIndex::Bottom as usize].set_attributes(reader, e, true);
                    }
                    b"diagonal" => {
                        self.data[BordersIndex::Diagonal as usize].set_attributes(reader, e, true);
                    }
                    b"vertical" => {
                        self.data[BordersIndex::Vertical as usize].set_attributes(reader, e, true);
                    }
                    b"horizontal" => {
                        self.data[BordersIndex::Horizontal as usize].set_attributes(reader, e, true);
                    }
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"left" => {
                        self.data[BordersIndex::Left as usize].set_attributes(reader, e, false);
                    }
                    b"right" => {
                        self.data[BordersIndex::Right as usize].set_attributes(reader, e, false);
                    }
                    b"top" => {
                        self.data[BordersIndex::Top as usize].set_attributes(reader, e, false);
                    }
                    b"bottom" => {
                        self.data[BordersIndex::Bottom as usize].set_attributes(reader, e, false);
                    }
                    b"diagonal" => {
                        self.data[BordersIndex::Diagonal as usize].set_attributes(reader, e, false);
                    }
                    b"vertical" => {
                        self.data[BordersIndex::Vertical as usize].set_attributes(reader, e, false);
                    }
                    b"horizontal" => {
                        self.data[BordersIndex::Horizontal as usize].set_attributes(reader, e, false);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"border" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "border")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // border
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.diagonal_up.has_value() {
            attributes.push(("diagonalUp", self.diagonal_up.value_string()).into());
        }
        if self.diagonal_down.has_value() {
            attributes.push(("diagonalDown", self.diagonal_down.value_string()).into());
        }
        write_start_tag(writer, "border", attributes, false);

        // left
        self.data[BordersIndex::Left as usize].write_to_left(writer);

        // right
        self.data[BordersIndex::Right as usize].write_to_right(writer);

        // top
        self.data[BordersIndex::Top as usize].write_to_top(writer);

        // bottom
        self.data[BordersIndex::Bottom as usize].write_to_bottom(writer);

        // diagonal
        self.data[BordersIndex::Diagonal as usize].write_to_diagonal(writer);

        // vertical
        if self.data[BordersIndex::Vertical as usize].hash_code()
            != Border::default().hash_code()
        {
            self.data[BordersIndex::Vertical as usize].write_to_vertical(writer);
        }

        // horizontal
        if self.data[BordersIndex::Horizontal as usize].hash_code() != Border::default().hash_code() {
            self.data[BordersIndex::Horizontal as usize].write_to_horizontal(writer);
        }

        write_end_tag(writer, "border");
    }
}
