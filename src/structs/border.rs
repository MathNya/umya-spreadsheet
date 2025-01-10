// left right top bottom
use super::BorderStyleValues;
use super::Color;
use super::EnumValue;
use crate::reader::driver::*;
use crate::writer::driver::*;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Border {
    color: Color,
    style: EnumValue<BorderStyleValues>,
}

impl Border {
    #[inline]
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    #[inline]
    pub fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    #[inline]
    pub fn set_color(&mut self, value: Color) -> &mut Self {
        self.color = value;
        self
    }

    #[inline]
    pub fn get_style(&self) -> &BorderStyleValues {
        self.style.get_value()
    }

    #[inline]
    pub fn set_style(&mut self, value: BorderStyleValues) -> &mut Self {
        self.style.set_value(value);
        self
    }

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

    #[inline]
    pub fn get_border_style(&self) -> &str {
        self.style.get_value_string()
    }
    #[inline]
    pub fn set_border_style<S: Into<String>>(&mut self, value: S) {
        self.style.set_value_string(value);
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                &self.style.get_value_string(),
                &self.get_color().get_hash_code()
            ))
        )
    }

    // When opened in software such as Excel, it is visually blank.
    #[inline]
    pub(crate) fn is_visually_empty(&self) -> bool {
        self.style.get_value() == &BorderStyleValues::None
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, style, "style");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"color" {
                    self.color.set_attributes(reader, e, true);
                }
            },
            Event::End(ref e) => {
                match e.name().into_inner() {
                    b"left" => return,
                    b"right" => return,
                    b"top" => return,
                    b"bottom" => return,
                    b"diagonal" => return,
                    b"vertical" => return,
                    b"horizontal" => return,
                    _ => (),
                }
            },
            Event::Eof => panic!(
                "Error: Could not find {} end element",
                "left,right,top,bottom,diagonal,vertical,horizontal"
            )
        );
    }

    #[inline]
    pub(crate) fn write_to_left(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "left");
    }

    #[inline]
    pub(crate) fn write_to_right(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "right");
    }

    #[inline]
    pub(crate) fn write_to_top(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "top");
    }

    #[inline]
    pub(crate) fn write_to_bottom(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "bottom");
    }

    #[inline]
    pub(crate) fn write_to_diagonal(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "diagonal");
    }

    #[inline]
    pub(crate) fn write_to_vertical(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "vertical");
    }

    #[inline]
    pub(crate) fn write_to_horizontal(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "horizontal");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // left,right,top,bottom,diagonal,vertical,horizontal
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.style.has_value() {
            attributes.push(("style", self.style.get_value_string()));
        }

        let empty_flag = !self.color.has_value();
        write_start_tag(writer, tag_name, attributes, empty_flag);

        if !empty_flag {
            // color
            self.color.write_to_color(writer);

            write_end_tag(writer, tag_name);
        }
    }
}
