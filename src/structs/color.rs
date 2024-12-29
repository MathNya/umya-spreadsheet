// color
use std::{
    borrow::Cow,
    io::Cursor,
};

use md5::Digest;
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};
use rgb::Argb;

use crate::{
    helper::color::calc_tint,
    reader::driver::get_attribute_value,
    structs::drawing::Theme,
    writer::driver::write_start_tag,
};

pub type ARGB8 = Argb<u8>;

const INDEXED_COLORS: [ARGB8; 56] = [
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x00,
    }, //  System Colour #1 - Black
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    }, //  System Colour #2 - White
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x00,
        b: 0x00,
    }, //  System Colour #3 - Red
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xFF,
        b: 0x00,
    }, //  System Colour #4 - Green
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0xFF,
    }, //  System Colour #5 - Blue
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
    }, //  System Colour #6 - Yellow
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x00,
        b: 0xFF,
    }, //  System Colour #7- Magenta
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xFF,
        b: 0xFF,
    }, //  System Colour #8- Cyan
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x00,
    }, //  Standard Colour #9
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x80,
        b: 0x00,
    }, //  Standard Colour #10
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x80,
    }, //  Standard Colour #11
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x80,
        b: 0x00,
    }, //  Standard Colour #12
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x80,
    }, //  Standard Colour #13
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x80,
        b: 0x80,
    }, //  Standard Colour #14
    ARGB8 {
        a: 0xFF,
        r: 0xC0,
        g: 0xC0,
        b: 0xC0,
    }, //  Standard Colour #15
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x80,
        b: 0x80,
    }, //  Standard Colour #16
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0x99,
        b: 0xFF,
    }, //  Chart Fill Colour #17
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0x33,
        b: 0x66,
    }, //  Chart Fill Colour #18
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0xCC,
    }, //  Chart Fill Colour #19
    ARGB8 {
        a: 0xFF,
        r: 0xCC,
        g: 0xFF,
        b: 0xFF,
    }, //  Chart Fill Colour #20
    ARGB8 {
        a: 0xFF,
        r: 0x66,
        g: 0x00,
        b: 0x66,
    }, //  Chart Fill Colour #21
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x80,
        b: 0x80,
    }, //  Chart Fill Colour #22
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x66,
        b: 0xCC,
    }, //  Chart Fill Colour #23
    ARGB8 {
        a: 0xFF,
        r: 0xCC,
        g: 0xCC,
        b: 0xFF,
    }, //  Chart Fill Colour #24
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x80,
    }, //  Chart Line Colour #25
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x00,
        b: 0xFF,
    }, //  Chart Line Colour #26
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
    }, //  Chart Line Colour #27
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xFF,
        b: 0xFF,
    }, //  Chart Line Colour #28
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x80,
    }, //  Chart Line Colour #29
    ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x00,
    }, //  Chart Line Colour #30
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x80,
        b: 0x80,
    }, //  Chart Line Colour #31
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0xFF,
    }, //  Chart Line Colour #32
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xCC,
        b: 0xFF,
    }, //  Standard Colour #33
    ARGB8 {
        a: 0xFF,
        r: 0xCC,
        g: 0xFF,
        b: 0xFF,
    }, //  Standard Colour #34
    ARGB8 {
        a: 0xFF,
        r: 0xCC,
        g: 0xFF,
        b: 0xCC,
    }, //  Standard Colour #35
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0x99,
    }, //  Standard Colour #36
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0xCC,
        b: 0xFF,
    }, //  Standard Colour #37
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x99,
        b: 0xCC,
    }, //  Standard Colour #38
    ARGB8 {
        a: 0xFF,
        r: 0xCC,
        g: 0x99,
        b: 0xFF,
    }, //  Standard Colour #39
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xCC,
        b: 0x99,
    }, //  Standard Colour #40
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0x66,
        b: 0xFF,
    }, //  Standard Colour #41
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0xCC,
        b: 0xCC,
    }, //  Standard Colour #42
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0xCC,
        b: 0x00,
    }, //  Standard Colour #43
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xCC,
        b: 0x00,
    }, //  Standard Colour #44
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x99,
        b: 0x00,
    }, //  Standard Colour #45
    ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x66,
        b: 0x00,
    }, //  Standard Colour #46
    ARGB8 {
        a: 0xFF,
        r: 0x66,
        g: 0x66,
        b: 0x99,
    }, //  Standard Colour #47
    ARGB8 {
        a: 0xFF,
        r: 0x96,
        g: 0x96,
        b: 0x96,
    }, //  Standard Colour #48
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x33,
        b: 0x66,
    }, //  Standard Colour #49
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0x99,
        b: 0x66,
    }, //  Standard Colour #50
    ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x33,
        b: 0x00,
    }, //  Standard Colour #51
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0x33,
        b: 0x00,
    }, //  Standard Colour #52
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0x33,
        b: 0x00,
    }, //  Standard Colour #53
    ARGB8 {
        a: 0xFF,
        r: 0x99,
        g: 0x33,
        b: 0x66,
    }, //  Standard Colour #54
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0x33,
        b: 0x99,
    }, //  Standard Colour #55
    ARGB8 {
        a: 0xFF,
        r: 0x33,
        g: 0x33,
        b: 0x33,
    }, // Standard Colour #56
];

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Color {
    indexed:     Option<u32>,
    theme_index: Option<u32>,
    argb:        Option<ARGB8>,
    tint:        Option<f64>,
}

impl Color {
    // Colors
    pub const COLOR_BLACK: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };
    pub const COLOR_BLACK_STR: &str = "FF000000";
    pub const COLOR_BLUE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0xFF,
    };
    pub const COLOR_BLUE_STR: &str = "FF0000FF";
    pub const COLOR_DARKBLUE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x80,
    };
    pub const COLOR_DARKBLUE_STR: &str = "FF000080";
    pub const COLOR_DARKGREEN: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x80,
        b: 0x00,
    };
    pub const COLOR_DARKGREEN_STR: &str = "FF008000";
    pub const COLOR_DARKRED: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x00,
    };
    pub const COLOR_DARKRED_STR: &str = "FF800000";
    pub const COLOR_DARKYELLOW: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x80,
        b: 0x00,
    };
    pub const COLOR_DARKYELLOW_STR: &str = "FF808000";
    pub const COLOR_GREEN: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xFF,
        b: 0x00,
    };
    pub const COLOR_GREEN_STR: &str = "FF00FF00";
    pub const COLOR_RED: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x00,
        b: 0x00,
    };
    pub const COLOR_RED_STR: &str = "FFFF0000";
    pub const COLOR_WHITE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };
    pub const COLOR_WHITE_STR: &str = "FFFFFFFF";
    pub const COLOR_YELLOW: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
    };
    pub const COLOR_YELLOW_STR: &str = "FFFFFF00";
    pub const NAMED_COLORS: &[&str] = &[
        "Black", "White", "Red", "Green", "Blue", "Yellow", "Magenta", "Cyan",
    ];

    /// Convert hex string to ARGB8
    #[must_use]
    pub fn hex_to_argb8(hex: &str) -> Option<ARGB8> {
        if hex.len() == 9 && hex.starts_with('#') {
            let a = u8::from_str_radix(&hex[1..3], 16).ok()?;
            let r = u8::from_str_radix(&hex[3..5], 16).ok()?;
            let g = u8::from_str_radix(&hex[5..7], 16).ok()?;
            let b = u8::from_str_radix(&hex[7..9], 16).ok()?;

            Some(ARGB8 { a, r, g, b })
        } else if hex.len() == 8 {
            let a = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let b = u8::from_str_radix(&hex[6..8], 16).ok()?;

            Some(ARGB8 { a, r, g, b })
        } else if hex.len() == 7 && hex.starts_with('#') {
            let a = 0xFF;
            let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
            let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
            let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

            Some(ARGB8 { a, r, g, b })
        } else if hex.len() == 6 {
            let a = 0xFF;
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

            Some(ARGB8 { a, r, g, b })
        } else if hex.len() < 6 {
            // To pass an integration test where the hex string is "#333".
            // https://github.com/MathNya/umya-spreadsheet/pull/113

            let padded_hex = format!("{:0<6}", hex.replace('#', ""));
            let a = 0xFF;
            let r = u8::from_str_radix(&padded_hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&padded_hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&padded_hex[4..6], 16).ok()?;

            Some(ARGB8 { a, r, g, b })
        } else {
            None
        }
    }

    /// Convert ARGB8 to hex string
    #[must_use]
    pub fn argb8_to_hex(argb: ARGB8) -> String {
        format!("{:02X}{:02X}{:02X}{:02X}", argb.a, argb.r, argb.g, argb.b)
    }

    #[must_use]
    pub fn get_argb(&self) -> ARGB8 {
        if let Some(idx) = self.indexed {
            if let Some(v) = INDEXED_COLORS.get(idx as usize) {
                return *v;
            }
        }
        self.argb.unwrap_or_default()
    }

    /// Get Argb.
    /// If the color is based on the theme, it cannot be obtained with this
    /// function. In that case, use `get_argb_with_theme(&self`, theme:
    /// &Theme).
    #[must_use]
    pub fn get_argb_str(&self) -> String {
        Self::argb8_to_hex(self.get_argb())
    }

    /// Get Argb.
    /// Color information based on the theme can also be obtained.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let theme = book.get_theme();
    /// ```
    #[must_use]
    pub fn get_argb_with_theme(&self, theme: &Theme) -> Cow<'static, str> {
        if self.indexed.is_some() {
            return self.get_argb_str().into();
        }
        if let Some(key) = self.theme_index {
            if let Some(v) = theme
                .get_theme_elements()
                .get_color_scheme()
                .get_color_map()
                .get(key as usize)
            {
                if let Some(tint) = self.tint {
                    return calc_tint(v, tint).into();
                }
                return v.to_string().into();
            }
        }
        self.get_argb_str().to_string().into()
    }

    pub fn set_argb<S: Into<ARGB8>>(&mut self, value: S) -> &mut Self {
        let argb = value.into();
        let indexed = INDEXED_COLORS.iter().position(|&r| r == argb);

        if let Some(v) = indexed {
            self.indexed = Some(u32::try_from(v).unwrap());
            self.argb = None;
        } else {
            self.indexed = None;
            self.argb = Some(argb);
        }
        self.theme_index = None;
        self
    }

    pub fn set_argb_str<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let argb = Self::hex_to_argb8(value.as_ref()).unwrap();
        let indexed = INDEXED_COLORS.iter().position(|&r| r == argb);

        if let Some(v) = indexed {
            self.indexed = Some(u32::try_from(v).unwrap());
            self.argb = None;
        } else {
            self.indexed = None;
            self.argb = Some(argb);
        }
        self.theme_index = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_indexed(&self) -> u32 {
        self.indexed.unwrap_or(0)
    }

    #[inline]
    pub fn set_indexed(&mut self, index: u32) -> &mut Self {
        self.indexed = Some(index);
        self.theme_index = None;
        self.argb = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_theme_index(&self) -> u32 {
        self.theme_index.unwrap_or(0)
    }

    #[inline]
    pub fn set_theme_index(&mut self, index: u32) -> &mut Self {
        self.indexed = None;
        self.theme_index = Some(index);
        self.argb = None;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_tint(&self) -> f64 {
        self.tint.unwrap_or(0.0)
    }

    #[inline]
    pub fn set_tint(&mut self, value: f64) -> &mut Color {
        self.tint = Some(value);
        self
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.theme_index.is_some()
            || self.indexed.is_some()
            || self.argb.is_some()
            || self.tint.is_some()
    }

    #[inline]
    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}",
                self.indexed.map_or(String::new(), |v| v.to_string()),
                self.theme_index.map_or(String::new(), |v| v.to_string()),
                self.argb.map_or(String::new(), Self::argb8_to_hex),
                self.tint.map_or(String::new(), |v| v.to_string())
            ))
        )
    }

    // When opened in software such as Excel, it is visually blank.
    #[inline]
    pub(crate) fn is_visually_empty(&self) -> bool {
        !self.has_value()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        for attr in e.attributes().with_checks(false).flatten() {
            match attr.key.0 {
                b"indexed" => {
                    if let Ok(v) = get_attribute_value(&attr) {
                        if let Ok(num) = v.parse() {
                            self.indexed = Some(num);
                        }
                    }
                }
                b"theme" => {
                    if let Ok(v) = get_attribute_value(&attr) {
                        if let Ok(num) = v.parse() {
                            self.theme_index = Some(num);
                        }
                    }
                }
                b"rgb" => {
                    if let Ok(v) = get_attribute_value(&attr) {
                        self.argb = Self::hex_to_argb8(&v);
                    }
                }
                b"tint" => {
                    if let Ok(v) = get_attribute_value(&attr) {
                        if let Ok(num) = v.parse() {
                            self.tint = Some(num);
                        }
                    }
                }
                _ => {}
            }
        }

        if empty_flg {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"color" | b"fgColor" | b"bgColor" | b"tabColor" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!(
                    "Error: Could not find {} end element",
                    "color,fgColor,bgColor,tabColor"
                ),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    #[inline]
    pub(crate) fn write_to_color(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // color
        self.write_to(writer, "color");
    }

    #[inline]
    pub(crate) fn write_to_fg_color(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // fgColor
        self.write_to(writer, "fgColor");
    }

    #[inline]
    pub(crate) fn write_to_bg_color(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // bgColor
        self.write_to(writer, "bgColor");
    }

    #[inline]
    pub(crate) fn write_to_tab_color(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // tabColor
        self.write_to(writer, "tabColor");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        if let Some(theme_index) = self.theme_index {
            attributes.push(("theme", theme_index.to_string()).into());
        } else if let Some(indexed) = self.indexed {
            attributes.push(("indexed", indexed.to_string()).into());
        } else if let Some(argb) = self.argb {
            attributes.push(("rgb", Self::argb8_to_hex(argb)).into());
        }

        if let Some(tint) = self.tint {
            attributes.push(("tint", tint.to_string()).into());
        }

        if !attributes.is_empty() {
            write_start_tag(writer, tag_name, attributes, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let hex = "FF123456";
        let argb = Color::hex_to_argb8(hex).unwrap();
        assert_eq!(Color::argb8_to_hex(argb), hex);
    }

    #[test]
    fn set_value() {
        let mut obj = Color::default();
        obj.set_argb_str("F34F8080");
        assert_eq!(obj.get_argb_str(), "F34F8080");

        let mut obj = Color::default();
        obj.set_argb_str("FFFF8080");
        assert_eq!(obj.get_indexed(), 21);
        assert_eq!(obj.get_argb_str(), "FFFF8080");

        let mut obj = Color::default();
        let theme = Theme::get_default_value();
        obj.set_theme_index(1);
        assert_eq!(obj.get_argb_with_theme(&theme), "000000");
    }
}
