// border
use std::io::Cursor;

use md5::Digest;
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

use paste::paste;

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

macro_rules! border_accessors {
    ($($fn_name:ident, $index:ident),*) => {
        impl Borders {
            paste! {
                $(
                    pub fn [<get_ $fn_name _border>](&self) -> &Border {
                        &self.data[BordersIndex::$index as usize]
                    }

                    pub fn [<get_ $fn_name _border_mut>](&mut self) -> &mut Border {
                        &mut self.data[BordersIndex::$index as usize]
                    }

                    pub fn [<set_ $fn_name _border>](&mut self, value: Border) -> &mut Self {
                        self.data[BordersIndex::$index as usize] = value;
                        self
                    }

                    pub fn [<get_ $fn_name>](&self) -> &Border {
                        &self.data[BordersIndex::$index as usize]
                    }

                    pub fn [<get_ $fn_name _mut>](&mut self) -> &mut Border {
                        &mut self.data[BordersIndex::$index as usize]
                    }

                    pub fn [<set_ $fn_name>](&mut self, value: Border) -> &mut Self {
                        self.data[BordersIndex::$index as usize] = value;
                        self
                    }
                )*
            }
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

border_accessors! {
    left, Left,
    right, Right,
    top, Top,
    bottom, Bottom,
    diagonal, Diagonal,
    vertical, Vertical,
    horizontal, Horizontal
}

impl Borders {
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
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}{}{}{}{}{}",
                &self.get_left_border().hash_code(),
                &self.get_right_border().hash_code(),
                &self.get_top_border().hash_code(),
                &self.get_bottom_border().hash_code(),
                &self.get_diagonal_border().hash_code(),
                &self.get_vertical_border().hash_code(),
                &self.get_horizontal_border().hash_code(),
                &self.diagonal_down.value_string(),
                &self.diagonal_up.value_string()
            ))
        )
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
