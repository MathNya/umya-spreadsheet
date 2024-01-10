use md5::Digest;
use structs::Alignment;
use structs::Borders;
use structs::Color;
use structs::Fill;
use structs::Font;
use structs::NumberingFormat;
use structs::PatternValues;
use structs::Protection;
use structs::UInt32Value;

use crate::BooleanValue;

/// # Examples
/// ## add border
/// ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/style/style_border.png)
/// ```rust
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let mut style = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("D2");
///
/// // add bottom border
/// style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_MEDIUM);
/// // add top border
/// style.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_MEDIUM);
/// // add left border
/// style.get_borders_mut().get_left_mut().set_border_style(Border::BORDER_MEDIUM);
/// // add right border
/// style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_MEDIUM);
/// ```
///
/// ## change cell color
/// ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/style/style_fill_color.png)
/// ```rust
/// use umya_spreadsheet::*;
///
/// let mut book = new_file();
/// let mut style = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("A1");
///
/// // fill color on red.
/// style.set_background_color(Color::COLOR_RED);
/// ```
///
/// ## change font color
/// ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/style/style_font_color.png)
/// ```rust
/// use umya_spreadsheet::*;
///
/// let mut book = new_file();
/// let mut style = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("A1");
///
/// // font color on red.
/// style.get_font_mut().get_color_mut().set_argb(Color::COLOR_RED);
/// ```
#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Style {
    font: Option<Font>,
    fill: Option<Fill>,
    borders: Option<Borders>,
    alignment: Option<Alignment>,
    numbering_format: Option<NumberingFormat>,
    format_id: UInt32Value,
    protection: Option<Protection>,
}
impl Style {
    pub fn get_font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    pub fn get_font_mut(&mut self) -> &mut Font {
        self.font.get_or_insert(Font::get_default_value())
    }

    pub fn set_font(&mut self, value: Font) -> &mut Self {
        self.font = Some(value);
        self
    }

    pub fn remove_font(&mut self) -> &mut Self {
        self.font = None;
        self
    }

    pub(crate) fn set_font_crate(&mut self, value: Option<Font>) -> &mut Self {
        self.font = value;
        self
    }

    pub fn get_fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    pub fn get_fill_mut(&mut self) -> &mut Fill {
        self.fill.get_or_insert(Fill::get_default_value())
    }

    pub fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub fn get_background_color(&self) -> Option<&Color> {
        self.get_fill()
            .and_then(|fill| fill.get_pattern_fill()?.get_foreground_color())
    }

    pub fn set_background_color<S: Into<String>>(&mut self, color: S) -> &mut Self {
        self.set_background_color_solid(color);
        self
    }

    pub fn set_background_color_solid<S: Into<String>>(&mut self, color: S) -> &mut Self {
        self.get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(PatternValues::Solid)
            .remove_background_color()
            .get_foreground_color_mut()
            .set_argb(color);
        self
    }

    pub fn set_background_color_with_pattern<S: Into<String>>(
        &mut self,
        color1: S,
        color2: S,
        pattern: PatternValues,
    ) -> &mut Self {
        self.get_fill_mut()
            .get_pattern_fill_mut()
            .set_pattern_type(pattern)
            .get_background_color_mut()
            .set_argb(color1);
        self.get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .set_argb(color2);
        self
    }

    pub fn remove_fill(&mut self) -> &mut Self {
        self.fill = None;
        self
    }

    pub(crate) fn set_fill_crate(&mut self, value: Option<Fill>) -> &mut Self {
        self.fill = value;
        self
    }

    pub fn get_borders(&self) -> Option<&Borders> {
        self.borders.as_ref()
    }

    pub fn get_borders_mut(&mut self) -> &mut Borders {
        self.borders.get_or_insert(Borders::get_default_value())
    }

    pub fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders = Some(value);
        self
    }

    pub fn remove_borders(&mut self) -> &mut Self {
        self.borders = None;
        self
    }

    pub(crate) fn set_borders_crate(&mut self, value: Option<Borders>) -> &mut Self {
        self.borders = value;
        self
    }

    pub fn get_alignment(&self) -> Option<&Alignment> {
        self.alignment.as_ref()
    }

    pub fn get_alignment_mut(&mut self) -> &mut Alignment {
        self.alignment.get_or_insert(Alignment::default())
    }

    pub fn set_alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    pub fn remove_alignment(&mut self) -> &mut Self {
        self.alignment = None;
        self
    }

    pub(crate) fn set_alignment_crate(&mut self, value: Option<Alignment>) -> &mut Self {
        self.alignment = value;
        self
    }

    pub fn get_numbering_format(&self) -> Option<&NumberingFormat> {
        self.numbering_format.as_ref()
    }

    pub fn get_numbering_format_mut(&mut self) -> &mut NumberingFormat {
        self.numbering_format
            .get_or_insert(NumberingFormat::default())
    }

    pub fn set_numbering_format(&mut self, value: NumberingFormat) -> &mut Self {
        self.numbering_format = Some(value);
        self
    }

    pub fn remove_numbering_format(&mut self) -> &mut Self {
        self.numbering_format = None;
        self
    }

    pub fn get_number_format(&self) -> Option<&NumberingFormat> {
        self.get_numbering_format()
    }

    pub fn get_number_format_mut(&mut self) -> &mut NumberingFormat {
        self.get_numbering_format_mut()
    }

    pub fn set_number_format(&mut self, value: NumberingFormat) -> &mut Self {
        self.set_numbering_format(value)
    }

    pub fn remove_number_format(&mut self) -> &mut Self {
        self.remove_numbering_format()
    }

    pub fn get_format_id(&self) -> &u32 {
        self.format_id.get_value()
    }

    pub fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    pub fn get_protection(&self) -> Option<&Protection> {
        self.protection.as_ref()
    }

    pub fn get_protection_mut(&mut self) -> &mut Protection {
        self.protection.get_or_insert(Protection::default())
    }

    pub fn set_protection(&mut self, value: Protection) -> &mut Self {
        self.protection = Some(value);
        self
    }

    pub fn remove_protection(&mut self) -> &mut Self {
        self.protection = None;
        self
    }

    pub(crate) fn set_protection_crate(&mut self, value: Option<Protection>) -> &mut Self {
        self.protection = value;
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        !(self.font.is_some()
            || self.fill.is_some()
            || self.borders.is_some()
            || self.alignment.is_some()
            || self.numbering_format.is_some()
            || self.protection.is_some())
    }

    pub(crate) fn get_default_value() -> Self {
        let mut def = Self::default();
        def.set_font(Font::get_default_value());
        def.set_borders(Borders::get_default_value());
        def.set_fill(Fill::get_default_value());
        def
    }

    pub(crate) fn get_default_value_2() -> Self {
        let mut def = Self::default();
        def.set_font(Font::get_default_value());
        def.set_borders(Borders::get_default_value());
        def.set_fill(Fill::get_default_value_2());
        def
    }
}
