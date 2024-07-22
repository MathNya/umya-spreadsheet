// front
use super::Bold;
use super::Color;
use super::FontCharSet;
use super::FontFamilyNumbering;
use super::FontName;
use super::FontScheme;
use super::FontSchemeValues;
use super::FontSize;
use super::Italic;
use super::Strike;
use super::Underline;
use super::UnderlineValues;
use super::VerticalTextAlignment;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use std::str::FromStr;
use writer::driver::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Font {
    font_name: FontName,
    font_size: FontSize,
    font_family_numbering: FontFamilyNumbering,
    font_bold: Bold,
    font_italic: Italic,
    font_underline: Underline,
    font_strike: Strike,
    color: Color,
    font_char_set: FontCharSet,
    font_scheme: FontScheme,
    vertical_text_alignment: VerticalTextAlignment,
}
impl Font {
    // Charset
    pub const CHARSET_ANSI: i32 = 0;
    pub const CHARSET_DEFAULT: i32 = 1;
    pub const CHARSET_SYMBOL: i32 = 2;
    pub const CHARSET_SHIFTJIS: i32 = 128;
    pub const CHARSET_HANGEUL: i32 = 129;
    pub const CHARSET_HANGUL: i32 = 129;
    pub const CHARSET_GB2312: i32 = 134;
    pub const CHARSET_CHINESEBIG5: i32 = 136;
    pub const CHARSET_OEM: i32 = 255;
    pub const CHARSET_JOHAB: i32 = 130;
    pub const CHARSET_HEBREW: i32 = 177;
    pub const CHARSET_ARABIC: i32 = 178;
    pub const CHARSET_GREEK: i32 = 161;
    pub const CHARSET_TURKISH: i32 = 162;
    pub const CHARSET_VIETNAMESE: i32 = 163;
    pub const CHARSET_THAI: i32 = 222;
    pub const CHARSET_EASTEUROPE: i32 = 238;
    pub const CHARSET_RUSSIAN: i32 = 204;
    pub const CHARSET_MAC: i32 = 77;
    pub const CHARSET_BALTIC: i32 = 186;

    // Underline types
    pub const UNDERLINE_NONE: &'static str = "none";
    pub const UNDERLINE_DOUBLE: &'static str = "double";
    pub const UNDERLINE_DOUBLEACCOUNTING: &'static str = "doubleAccounting";
    pub const UNDERLINE_SINGLE: &'static str = "single";
    pub const UNDERLINE_SINGLEACCOUNTING: &'static str = "singleAccounting";

    pub fn get_font_name(&self) -> &FontName {
        &self.font_name
    }

    pub fn get_font_name_mut(&mut self) -> &mut FontName {
        &mut self.font_name
    }

    pub fn set_font_name(&mut self, value: FontName) -> &mut Self {
        self.font_name = value;
        self
    }

    pub fn get_name(&self) -> &str {
        self.font_name.get_val()
    }

    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.font_name.set_val(value);
        self.set_scheme("none");
        self
    }

    pub fn set_name_with_scheme<S: Into<String>>(&mut self, name: S, scheme: S) -> &mut Self {
        self.set_name(name);
        self.set_scheme(scheme);
        self
    }

    pub fn get_font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn get_font_size_mut(&mut self) -> &mut FontSize {
        &mut self.font_size
    }

    pub fn set_font_size(&mut self, value: FontSize) -> &mut Self {
        self.font_size = value;
        self
    }

    pub fn get_size(&self) -> &f64 {
        self.font_size.get_val()
    }

    pub fn set_size(&mut self, value: f64) -> &mut Self {
        self.font_size.set_val(value);
        self
    }

    pub fn get_font_family_numbering(&self) -> &FontFamilyNumbering {
        &self.font_family_numbering
    }

    pub fn get_font_family_numbering_mut(&mut self) -> &mut FontFamilyNumbering {
        &mut self.font_family_numbering
    }

    pub fn set_font_family_numbering(&mut self, value: FontFamilyNumbering) -> &mut Self {
        self.font_family_numbering = value;
        self
    }

    pub fn get_family(&self) -> &i32 {
        self.font_family_numbering.get_val()
    }

    pub fn set_family(&mut self, value: i32) -> &mut Self {
        self.font_family_numbering.set_val(value);
        self
    }

    pub fn get_font_bold(&self) -> &Bold {
        &self.font_bold
    }

    pub fn get_font_bold_mut(&mut self) -> &mut Bold {
        &mut self.font_bold
    }

    pub fn set_font_bold(&mut self, value: Bold) -> &mut Self {
        self.font_bold = value;
        self
    }

    pub fn get_bold(&self) -> &bool {
        self.font_bold.get_val()
    }

    pub fn set_bold(&mut self, value: bool) -> &mut Self {
        self.font_bold.set_val(value);
        self
    }

    pub fn get_font_italic(&self) -> &Italic {
        &self.font_italic
    }

    pub fn get_font_italic_mut(&mut self) -> &mut Italic {
        &mut self.font_italic
    }

    pub fn set_font_italic(&mut self, value: Italic) -> &mut Self {
        self.font_italic = value;
        self
    }

    pub fn get_italic(&self) -> &bool {
        self.font_italic.get_val()
    }

    pub fn set_italic(&mut self, value: bool) -> &mut Self {
        self.font_italic.set_val(value);
        self
    }

    pub fn get_font_underline(&self) -> &Underline {
        &self.font_underline
    }

    pub fn get_font_underline_mut(&mut self) -> &mut Underline {
        &mut self.font_underline
    }

    pub fn set_font_underline(&mut self, value: Underline) -> &mut Self {
        self.font_underline = value;
        self
    }

    pub fn get_underline(&self) -> &str {
        self.font_underline.val.get_value_string()
    }

    pub fn set_underline<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let obj = value.into();
        self.font_underline
            .set_val(UnderlineValues::from_str(&obj).unwrap());
        self
    }

    pub fn get_font_strike(&self) -> &Strike {
        &self.font_strike
    }

    pub fn get_font_strike_mut(&mut self) -> &mut Strike {
        &mut self.font_strike
    }

    pub fn set_font_strike(&mut self, value: Strike) -> &mut Self {
        self.font_strike = value;
        self
    }

    pub fn get_strikethrough(&self) -> &bool {
        self.font_strike.get_val()
    }

    pub fn set_strikethrough(&mut self, value: bool) -> &mut Self {
        self.font_strike.set_val(value);
        self
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    pub fn set_color(&mut self, value: Color) -> &mut Self {
        self.color = value;
        self
    }

    pub fn get_font_char_set(&self) -> &FontCharSet {
        &self.font_char_set
    }

    pub fn get_font_char_set_mut(&mut self) -> &mut FontCharSet {
        &mut self.font_char_set
    }

    pub fn set_font_char_set(&mut self, value: FontCharSet) -> &mut Self {
        self.font_char_set = value;
        self
    }

    pub fn get_charset(&self) -> &i32 {
        self.font_char_set.get_val()
    }

    pub fn set_charset(&mut self, value: i32) -> &mut Self {
        self.font_char_set.set_val(value);
        self
    }

    pub fn get_font_scheme(&self) -> &FontScheme {
        &self.font_scheme
    }

    pub fn get_font_scheme_mut(&mut self) -> &mut FontScheme {
        &mut self.font_scheme
    }

    pub fn set_font_scheme(&mut self, value: FontScheme) -> &mut Self {
        self.font_scheme = value;
        self
    }

    pub fn get_scheme(&self) -> &str {
        self.font_scheme.val.get_value_string()
    }

    pub fn set_scheme<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let obj = value.into();
        self.font_scheme
            .set_val(FontSchemeValues::from_str(&obj).unwrap());
        self
    }

    pub fn get_vertical_text_alignment(&self) -> &VerticalTextAlignment {
        &self.vertical_text_alignment
    }

    pub fn get_vertical_text_alignment_mut(&mut self) -> &mut VerticalTextAlignment {
        &mut self.vertical_text_alignment
    }

    pub fn set_vertical_text_alignment(&mut self, value: VerticalTextAlignment) -> &mut Self {
        self.vertical_text_alignment = value;
        self
    }

    pub(crate) fn get_default_value() -> Self {
        let mut def = Self::default();
        def.set_size(11.0);
        def.set_name_with_scheme("Calibri", "minor");
        def.get_color_mut().set_theme_index(1);
        def.set_family(2);
        def
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}{}{}{}{}{}{}{}",
                &self.font_name.val.get_hash_string(),
                &self.font_size.val.get_hash_string(),
                &self.font_family_numbering.val.get_hash_string(),
                &self.font_bold.val.get_hash_string(),
                &self.font_italic.val.get_hash_string(),
                &self.font_underline.val.get_hash_string(),
                &self.font_strike.val.get_hash_string(),
                &self.color.get_hash_code(),
                &self.font_char_set.val.get_hash_string(),
                &self.font_scheme.val.get_hash_string(),
                &self.vertical_text_alignment.val.get_hash_string(),
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"name" => {
                        self.font_name.set_attributes(reader, e);
                    }
                    b"sz" => {
                        self.font_size.set_attributes(reader, e);
                    }
                    b"family" => {
                        self.font_family_numbering.set_attributes(reader, e);
                    }
                    b"b" => {
                        self.font_bold.set_attributes(reader, e);
                    }
                    b"i" => {
                        self.font_italic.set_attributes(reader, e);
                    }
                    b"u" => {
                        self.font_underline.set_attributes(reader, e);
                    }
                    b"strike" => {
                        self.font_strike.set_attributes(reader, e);
                    }
                    b"color" => {
                        self.color.set_attributes(reader, e, true);
                    }
                    b"charset" => {
                        self.font_char_set.set_attributes(reader, e);
                    }
                    b"scheme" => {
                        self.font_scheme.set_attributes(reader, e);
                    }
                    b"vertAlign" => {
                        self.vertical_text_alignment.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"font" => return,
                    b"rPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error: Could not find {} end element", "font, rPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to_font(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // font
        self.write_to(writer, "font", "name");
    }

    pub(crate) fn write_to_rpr(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // rPr
        self.write_to(writer, "rPr", "rFont");
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        tag_name: &str,
        tag_font_name: &str,
    ) {
        // font
        write_start_tag(writer, tag_name, vec![], false);

        // bold
        self.font_bold.write_to(writer);

        // italic
        self.font_italic.write_to(writer);

        // underline
        self.font_underline.write_to(writer);

        // strike
        self.font_strike.write_to(writer);

        // vertAlign
        self.vertical_text_alignment.write_to(writer);

        // sz
        self.font_size.write_to(writer);

        // color
        self.color.write_to_color(writer);

        // name
        self.font_name.write_to(writer, tag_font_name);

        // family
        self.font_family_numbering.write_to(writer);

        // charset
        self.font_char_set.write_to(writer);

        // scheme
        self.font_scheme.write_to(writer);

        write_end_tag(writer, tag_name);
    }
}
