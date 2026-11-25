use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::BooleanValue;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PrintOptions {
    horizontal_centered: BooleanValue,
    vertical_centered:   BooleanValue,
    headings:            BooleanValue,
    grid_lines:          BooleanValue,
    grid_lines_set:      BooleanValue,
}

impl PrintOptions {
    #[inline]
    #[must_use]
    pub fn horizontal_centered(&self) -> bool {
        self.horizontal_centered.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use horizontal_centered()")]
    pub fn get_horizontal_centered(&self) -> bool {
        self.horizontal_centered()
    }

    #[inline]
    pub fn set_horizontal_centered(&mut self, value: bool) -> &mut Self {
        self.horizontal_centered.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn vertical_centered(&self) -> bool {
        self.vertical_centered.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use vertical_centered()")]
    pub fn get_vertical_centered(&self) -> bool {
        self.vertical_centered()
    }

    #[inline]
    pub fn set_vertical_centered(&mut self, value: bool) -> &mut Self {
        self.vertical_centered.set_value(value);
        self
    }

    /// Whether the row and column headings are printed (`headings`).
    #[inline]
    #[must_use]
    pub fn headings(&self) -> bool {
        self.headings.value()
    }

    #[inline]
    pub fn set_headings(&mut self, value: bool) -> &mut Self {
        self.headings.set_value(value);
        self
    }

    /// Whether the grid lines are printed (`gridLines`).
    #[inline]
    #[must_use]
    pub fn grid_lines(&self) -> bool {
        self.grid_lines.value()
    }

    #[inline]
    pub fn set_grid_lines(&mut self, value: bool) -> &mut Self {
        self.grid_lines.set_value(value);
        self
    }

    /// Whether `gridLines` has been set (`gridLinesSet`). Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn grid_lines_set(&self) -> bool {
        !self.grid_lines_set.has_value() || self.grid_lines_set.value()
    }

    #[inline]
    pub fn set_grid_lines_set(&mut self, value: bool) -> &mut Self {
        self.grid_lines_set.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.horizontal_centered.has_value()
            || self.vertical_centered.has_value()
            || self.headings.has_value()
            || self.grid_lines.has_value()
            || self.grid_lines_set.has_value()
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, horizontal_centered, "horizontalCentered");
        set_string_from_xml!(self, e, vertical_centered, "verticalCentered");
        set_string_from_xml!(self, e, headings, "headings");
        set_string_from_xml!(self, e, grid_lines, "gridLines");
        set_string_from_xml!(self, e, grid_lines_set, "gridLinesSet");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // printOptions
            let mut attributes: crate::structs::AttrCollection = Vec::new();
            if self.horizontal_centered.has_value() {
                attributes.push(
                    (
                        "horizontalCentered",
                        self.horizontal_centered.value_string(),
                    )
                        .into(),
                );
            }
            if self.vertical_centered.has_value() {
                attributes.push(("verticalCentered", self.vertical_centered.value_string()).into());
            }
            if self.headings.has_value() {
                attributes.push(("headings", self.headings.value_string()).into());
            }
            if self.grid_lines.has_value() {
                attributes.push(("gridLines", self.grid_lines.value_string()).into());
            }
            if self.grid_lines_set.has_value() {
                attributes.push(("gridLinesSet", self.grid_lines_set.value_string()).into());
            }
            write_start_tag(writer, "printOptions", attributes, true);
        }
    }
}
