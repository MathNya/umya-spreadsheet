// tabColor
use super::DoubleValue;
use super::StringValue;
use super::Theme;
use super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

const INDEXED_COLORS: &'static [&'static str] = &[
    "FF000000", //  System Colour #1 - Black
    "FFFFFFFF", //  System Colour #2 - White
    "FFFF0000", //  System Colour #3 - Red
    "FF00FF00", //  System Colour #4 - Green
    "FF0000FF", //  System Colour #5 - Blue
    "FFFFFF00", //  System Colour #6 - Yellow
    "FFFF00FF", //  System Colour #7- Magenta
    "FF00FFFF", //  System Colour #8- Cyan
    "FF800000", //  Standard Colour #9
    "FF008000", //  Standard Colour #10
    "FF000080", //  Standard Colour #11
    "FF808000", //  Standard Colour #12
    "FF800080", //  Standard Colour #13
    "FF008080", //  Standard Colour #14
    "FFC0C0C0", //  Standard Colour #15
    "FF808080", //  Standard Colour #16
    "FF9999FF", //  Chart Fill Colour #17
    "FF993366", //  Chart Fill Colour #18
    "FFFFFFCC", //  Chart Fill Colour #19
    "FFCCFFFF", //  Chart Fill Colour #20
    "FF660066", //  Chart Fill Colour #21
    "FFFF8080", //  Chart Fill Colour #22
    "FF0066CC", //  Chart Fill Colour #23
    "FFCCCCFF", //  Chart Fill Colour #24
    "FF000080", //  Chart Line Colour #25
    "FFFF00FF", //  Chart Line Colour #26
    "FFFFFF00", //  Chart Line Colour #27
    "FF00FFFF", //  Chart Line Colour #28
    "FF800080", //  Chart Line Colour #29
    "FF800000", //  Chart Line Colour #30
    "FF008080", //  Chart Line Colour #31
    "FF0000FF", //  Chart Line Colour #32
    "FF00CCFF", //  Standard Colour #33
    "FFCCFFFF", //  Standard Colour #34
    "FFCCFFCC", //  Standard Colour #35
    "FFFFFF99", //  Standard Colour #36
    "FF99CCFF", //  Standard Colour #37
    "FFFF99CC", //  Standard Colour #38
    "FFCC99FF", //  Standard Colour #39
    "FFFFCC99", //  Standard Colour #40
    "FF3366FF", //  Standard Colour #41
    "FF33CCCC", //  Standard Colour #42
    "FF99CC00", //  Standard Colour #43
    "FFFFCC00", //  Standard Colour #44
    "FFFF9900", //  Standard Colour #45
    "FFFF6600", //  Standard Colour #46
    "FF666699", //  Standard Colour #47
    "FF969696", //  Standard Colour #48
    "FF003366", //  Standard Colour #49
    "FF339966", //  Standard Colour #50
    "FF003300", //  Standard Colour #51
    "FF333300", //  Standard Colour #52
    "FF993300", //  Standard Colour #53
    "FF993366", //  Standard Colour #54
    "FF333399", //  Standard Colour #55
    "FF333333", //  Standard Colour #56
];

#[derive(Default, Debug, Clone)]
pub struct TabColor {
    indexed: UInt32Value,
    theme_index: UInt32Value,
    argb: StringValue,
    tint: DoubleValue,
}
impl TabColor {
    pub const NAMED_COLORS: &'static [&'static str] = &[
        "Black", "White", "Red", "Green", "Blue", "Yellow", "Magenta", "Cyan",
    ];

    // Colors
    pub const COLOR_BLACK: &'static str = "FF000000";
    pub const COLOR_WHITE: &'static str = "FFFFFFFF";
    pub const COLOR_RED: &'static str = "FFFF0000";
    pub const COLOR_DARKRED: &'static str = "FF800000";
    pub const COLOR_BLUE: &'static str = "FF0000FF";
    pub const COLOR_DARKBLUE: &'static str = "FF000080";
    pub const COLOR_GREEN: &'static str = "FF00FF00";
    pub const COLOR_DARKGREEN: &'static str = "FF008000";
    pub const COLOR_YELLOW: &'static str = "FFFFFF00";
    pub const COLOR_DARKYELLOW: &'static str = "FF808000";

    #[inline]
    pub fn get_argb(&self) -> &str {
        &self.argb.get_value()
    }

    #[inline]
    pub fn set_argb<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.indexed.remove_value();
        self.theme_index.remove_value();
        self.argb.set_value(value);
        self
    }

    #[inline]
    pub fn get_indexed(&self) -> &u32 {
        &self.indexed.get_value()
    }

    pub fn set_indexed(&mut self, index: u32) -> &mut Self {
        self.indexed.set_value(index);
        self.theme_index.remove_value();
        self.argb
            .set_value(match INDEXED_COLORS.get(index as usize - 1) {
                Some(v) => v.to_string(),
                None => String::from(""),
            });
        self
    }

    #[inline]
    pub fn get_theme_index(&self) -> &u32 {
        &self.theme_index.get_value()
    }

    #[inline]
    pub fn set_theme_index(&mut self, index: u32) -> &mut Self {
        self.indexed.remove_value();
        self.theme_index.set_value(index);
        self.argb.remove_value();
        self
    }

    pub(crate) fn set_argb_by_theme(&mut self, theme: &Theme) -> &mut Self {
        if self.theme_index.has_value() {
            self.argb.set_value(
                match theme
                    .get_color_map()
                    .get(self.theme_index.get_value().clone() as usize)
                {
                    Some(v) => v.to_string(),
                    None => String::from(""),
                },
            );
        }
        self
    }

    #[inline]
    pub fn get_tint(&self) -> &f64 {
        &self.tint.get_value()
    }

    #[inline]
    pub fn set_tint(&mut self, value: f64) -> &mut Self {
        self.tint.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.theme_index.has_value()
            || self.indexed.has_value()
            || self.argb.has_value()
            || self.tint.has_value()
    }

    #[inline]
    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}",
                &self.indexed.get_hash_string(),
                &self.theme_index.get_hash_string(),
                &self.argb.get_hash_string(),
                &self.tint.get_hash_string()
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        for a in e.attributes().with_checks(false) {
            match a {
                Ok(ref attr) if attr.key.0 == b"indexed" => {
                    self.indexed
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.0 == b"theme" => {
                    self.theme_index
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.0 == b"rgb" => {
                    self.argb
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key.0 == b"tint" => {
                    self.tint
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // tabColor
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.theme_index.has_value() {
            attributes.push(("theme", self.theme_index.get_value_string()));
        } else if self.indexed.has_value() {
            attributes.push(("indexed", self.indexed.get_value_string()));
        } else if self.argb.has_value() {
            attributes.push(("rgb", self.argb.get_value_string()));
        }
        if self.tint.has_value() {
            attributes.push(("tint", self.tint.get_value_string()));
        }

        if attributes.len() > 0 {
            write_start_tag(writer, "tabColor", attributes, true);
        }
    }
}
