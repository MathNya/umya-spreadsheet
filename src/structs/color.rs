// color
use std::{
    borrow::Cow,
    io::Cursor,
};

use md5::Digest;
use phf::phf_map;
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

macro_rules! argb {
    ($a:expr, $r:expr, $g:expr, $b:expr) => {
        ARGB8 {
            a: $a,
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

static INDEX_TO_COLOR: phf::Map<u32, ARGB8> = phf_map! {
    0u32 => argb!(0xFF, 0x00, 0x00, 0x00), // System Colour #1 - Black
    1u32 => argb!(0xFF, 0xFF, 0xFF, 0xFF), // System Colour #2 - White
    2u32 => argb!(0xFF, 0xFF, 0x00, 0x00), // System Colour #3 - Red
    3u32 => argb!(0xFF, 0x00, 0xFF, 0x00), // System Colour #4 - Green
    4u32 => argb!(0xFF, 0x00, 0x00, 0xFF), // System Colour #5 - Blue
    5u32 => argb!(0xFF, 0xFF, 0xFF, 0x00), // System Colour #6 - Yellow
    6u32 => argb!(0xFF, 0xFF, 0x00, 0xFF), // System Colour #7- Magenta
    7u32 => argb!(0xFF, 0x00, 0xFF, 0xFF), // System Colour #8- Cyan
    8u32 => argb!(0xFF, 0x00, 0x00, 0x00), // System Colour #1 - Black
    9u32 => argb!(0xFF, 0xFF, 0xFF, 0xFF), // System Colour #2 - White
    10u32 => argb!(0xFF, 0xFF, 0x00, 0x00), // System Colour #3 - Red
    11u32 => argb!(0xFF, 0x00, 0xFF, 0x00), // System Colour #4 - Green
    12u32 => argb!(0xFF, 0x00, 0x00, 0xFF), // System Colour #5 - Blue
    13u32 => argb!(0xFF, 0xFF, 0xFF, 0x00), // System Colour #6 - Yellow
    14u32 => argb!(0xFF, 0xFF, 0x00, 0xFF), // System Colour #7- Magenta
    15u32 => argb!(0xFF, 0x00, 0xFF, 0xFF), // System Colour #8- Cyan
    16u32 => argb!(0xFF, 0x80, 0x00, 0x00), // Standard Colour #9
    17u32 => argb!(0xFF, 0x00, 0x80, 0x00), // Standard Colour #10
    18u32 => argb!(0xFF, 0x00, 0x00, 0x80), // Standard Colour #11
    19u32 => argb!(0xFF, 0x80, 0x80, 0x00), // Standard Colour #12
    20u32 => argb!(0xFF, 0x80, 0x00, 0x80), // Standard Colour #13
    21u32 => argb!(0xFF, 0x00, 0x80, 0x80), // Standard Colour #14
    22u32 => argb!(0xFF, 0xC0, 0xC0, 0xC0), // Standard Colour #15
    23u32 => argb!(0xFF, 0x80, 0x80, 0x80), // Standard Colour #16
    24u32 => argb!(0xFF, 0x99, 0x99, 0xFF), // Chart Fill Colour #17
    25u32 => argb!(0xFF, 0x99, 0x33, 0x66), // Chart Fill Colour #18
    26u32 => argb!(0xFF, 0xFF, 0xFF, 0xCC), // Chart Fill Colour #19
    27u32 => argb!(0xFF, 0xCC, 0xFF, 0xFF), // Chart Fill Colour #20
    28u32 => argb!(0xFF, 0x66, 0x00, 0x66), // Chart Fill Colour #21
    29u32 => argb!(0xFF, 0xFF, 0x80, 0x80), // Chart Fill Colour #22
    30u32 => argb!(0xFF, 0x00, 0x66, 0xCC), // Chart Fill Colour #23
    31u32 => argb!(0xFF, 0xCC, 0xCC, 0xFF), // Chart Fill Colour #24
    32u32 => argb!(0xFF, 0x00, 0x00, 0x80), // Chart Line Colour #25
    33u32 => argb!(0xFF, 0xFF, 0x00, 0xFF), // Chart Line Colour #26
    34u32 => argb!(0xFF, 0xFF, 0xFF, 0x00), // Chart Line Colour #27
    35u32 => argb!(0xFF, 0x00, 0xFF, 0xFF), // Chart Line Colour #28
    36u32 => argb!(0xFF, 0x80, 0x00, 0x80), // Chart Line Colour #29
    37u32 => argb!(0xFF, 0x80, 0x00, 0x00), // Chart Line Colour #30
    38u32 => argb!(0xFF, 0x00, 0x80, 0x80), // Chart Line Colour #31
    39u32 => argb!(0xFF, 0x00, 0x00, 0xFF), // Chart Line Colour #32
    40u32 => argb!(0xFF, 0x00, 0xCC, 0xFF), // Standard Colour #33
    41u32 => argb!(0xFF, 0xCC, 0xFF, 0xFF), // Standard Colour #34
    42u32 => argb!(0xFF, 0xCC, 0xFF, 0xCC), // Standard Colour #35
    43u32 => argb!(0xFF, 0xFF, 0xFF, 0x99), // Standard Colour #36
    44u32 => argb!(0xFF, 0x99, 0xCC, 0xFF), // Standard Colour #37
    45u32 => argb!(0xFF, 0xFF, 0x99, 0xCC), // Standard Colour #38
    46u32 => argb!(0xFF, 0xCC, 0x99, 0xFF), // Standard Colour #39
    47u32 => argb!(0xFF, 0xFF, 0xCC, 0x99), // Standard Colour #40
    48u32 => argb!(0xFF, 0x33, 0x66, 0xFF), // Standard Colour #41
    49u32 => argb!(0xFF, 0x33, 0xCC, 0xCC), // Standard Colour #42
    50u32 => argb!(0xFF, 0x99, 0xCC, 0x00), // Standard Colour #43
    51u32 => argb!(0xFF, 0xFF, 0xCC, 0x00), // Standard Colour #44
    52u32 => argb!(0xFF, 0xFF, 0x99, 0x00), // Standard Colour #45
    53u32 => argb!(0xFF, 0xFF, 0x66, 0x00), // Standard Colour #46
    54u32 => argb!(0xFF, 0x66, 0x66, 0x99), // Standard Colour #47
    55u32 => argb!(0xFF, 0x96, 0x96, 0x96), // Standard Colour #48
    56u32 => argb!(0xFF, 0x00, 0x33, 0x66), // Standard Colour #49
    57u32 => argb!(0xFF, 0x33, 0x99, 0x66), // Standard Colour #50
    58u32 => argb!(0xFF, 0x00, 0x33, 0x00), // Standard Colour #51
    59u32 => argb!(0xFF, 0x33, 0x33, 0x00), // Standard Colour #52
    60u32 => argb!(0xFF, 0x99, 0x33, 0x00), // Standard Colour #53
    61u32 => argb!(0xFF, 0x99, 0x33, 0x66), // Standard Colour #54
    62u32 => argb!(0xFF, 0x33, 0x33, 0x99), // Standard Colour #55
    63u32 => argb!(0xFF, 0x33, 0x33, 0x33), // Standard Colour #56
};

static COLOR_STR_TO_INDEX: phf::Map<&'static str, u32> = phf_map! {
    "FF000000" => 0u32, // System Colour #1 - Black
    "FFFFFFFF" => 1u32, // System Colour #2 - White
    "FFFF0000" => 2u32, // System Colour #3 - Red
    "FF00FF00" => 3u32, // System Colour #4 - Green
    "FF0000FF" => 4u32, // System Colour #5 - Blue
    "FFFFFF00" => 5u32, // System Colour #6 - Yellow
    "FFFF00FF" => 6u32, // System Colour #7- Magenta
    "FF00FFFF" => 7u32, // System Colour #8- Cyan
//    "FF000000" => 8u32, // System Colour #1 - Black - Duplicate Key !
//    "FFFFFFFF" => 9u32, // System Colour #2 - White - Duplicate Key !
//    "FFFF0000" => 10u32, // System Colour #3 - Red - Duplicate Key !
//    "FF00FF00" => 11u32, // System Colour #4 - Green - Duplicate Key !
//    "FF0000FF" => 12u32, // System Colour #5 - Blue - Duplicate Key !
//    "FFFFFF00" => 13u32, // System Colour #6 - Yellow - Duplicate Key !
//    "FFFF00FF" => 14u32, // System Colour #7- Magenta - Duplicate Key !
//    "FF00FFFF" => 15u32, // System Colour #8- Cyan - Duplicate Key !
    "FF800000" => 16u32, // Standard Colour #9
    "FF008000" => 17u32, // Standard Colour #10
    "FF000080" => 18u32, // Standard Colour #11
    "FF808000" => 19u32, // Standard Colour #12
    "FF800080" => 20u32, // Standard Colour #13
    "FF008080" => 21u32, // Standard Colour #14
    "FFC0C0C0" => 22u32, // Standard Colour #15
    "FF808080" => 23u32, // Standard Colour #16
    "FF9999FF" => 24u32, // Chart Fill Colour #17
    "FF993366" => 25u32, // Chart Fill Colour #18
    "FFFFFFCC" => 26u32, // Chart Fill Colour #19
    "FFCCFFFF" => 27u32, // Chart Fill Colour #20
    "FF660066" => 28u32, // Chart Fill Colour #21
    "FFFF8080" => 29u32, // Chart Fill Colour #22
    "FF0066CC" => 30u32, // Chart Fill Colour #23
    "FFCCCCFF" => 31u32, // Chart Fill Colour #24
//   "FF000080" => 32u32, // Chart Line Colour #25 - Duplicate Key !
//   "FFFF00FF" => 33u32, // Chart Line Colour #26 - Duplicate Key !
//   "FFFFFF00" => 34u32, // Chart Line Colour #27 - Duplicate Key !
//   "FF00FFFF" => 35u32, // Chart Line Colour #28 - Duplicate Key !
//   "FF800080" => 36u32, // Chart Line Colour #29 - Duplicate Key !
//   "FF800000" => 37u32, // Chart Line Colour #30 - Duplicate Key !
//   "FF008080" => 38u32, // Chart Line Colour #31 - Duplicate Key !
//   "FF0000FF" => 39u32, // Chart Line Colour #32 - Duplicate Key !
    "FF00CCFF" => 40u32, // Standard Colour #33
//   "FFCCFFFF" => 41u32, // Standard Colour #34 - Duplicate Key !
    "FFCCFFCC" => 42u32, // Standard Colour #35
    "FFFFFF99" => 43u32, // Standard Colour #36
    "FF99CCFF" => 44u32, // Standard Colour #37
    "FFFF99CC" => 45u32, // Standard Colour #38
    "FFCC99FF" => 46u32, // Standard Colour #39
    "FFFFCC99" => 47u32, // Standard Colour #40
    "FF3366FF" => 48u32, // Standard Colour #41
    "FF33CCCC" => 49u32, // Standard Colour #42
    "FF99CC00" => 50u32, // Standard Colour #43
    "FFFFCC00" => 51u32, // Standard Colour #44
    "FFFF9900" => 52u32, // Standard Colour #45
    "FFFF6600" => 53u32, // Standard Colour #46
    "FF666699" => 54u32, // Standard Colour #47
    "FF969696" => 55u32, // Standard Colour #48
    "FF003366" => 56u32, // Standard Colour #49
    "FF339966" => 57u32, // Standard Colour #50
    "FF003300" => 58u32, // Standard Colour #51
    "FF333300" => 59u32, // Standard Colour #52
    "FF993300" => 60u32, // Standard Colour #53
//    "FF993366" => 61u32, // Standard Colour #54 - Duplicate Key !
    "FF333399" => 62u32, // Standard Colour #55
    "FF333333" => 63u32, // Standard Colour #56
};

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
    pub const COLOR_BLACK_STR: &'static str = "FF000000";
    pub const COLOR_BLUE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0xFF,
    };
    pub const COLOR_BLUE_STR: &'static str = "FF0000FF";
    pub const COLOR_DARKBLUE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x00,
        b: 0x80,
    };
    pub const COLOR_DARKBLUE_STR: &'static str = "FF000080";
    pub const COLOR_DARKGREEN: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0x80,
        b: 0x00,
    };
    pub const COLOR_DARKGREEN_STR: &'static str = "FF008000";
    pub const COLOR_DARKRED: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x00,
        b: 0x00,
    };
    pub const COLOR_DARKRED_STR: &'static str = "FF800000";
    pub const COLOR_DARKYELLOW: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x80,
        g: 0x80,
        b: 0x00,
    };
    pub const COLOR_DARKYELLOW_STR: &'static str = "FF808000";
    pub const COLOR_GREEN: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0x00,
        g: 0xFF,
        b: 0x00,
    };
    pub const COLOR_GREEN_STR: &'static str = "FF00FF00";
    pub const COLOR_RED: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0x00,
        b: 0x00,
    };
    pub const COLOR_RED_STR: &'static str = "FFFF0000";
    pub const COLOR_WHITE: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };
    pub const COLOR_WHITE_STR: &'static str = "FFFFFFFF";
    pub const COLOR_YELLOW: ARGB8 = ARGB8 {
        a: 0xFF,
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
    };
    pub const COLOR_YELLOW_STR: &'static str = "FFFFFF00";
    pub const NAMED_COLORS: [&'static str; 8] = [
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
        } else if (hex.len() == 4 && hex.starts_with('#')) || hex.len() == 3 {
            // To pass an integration test where the hex string is "#333".
            // https://github.com/MathNya/umya-spreadsheet/pull/113
            // https://github.com/MathNya/umya-spreadsheet/pull/250#issuecomment-2566258423
            // https://www.w3schools.com/css/css_colors_hex.asp

            let padded_hex = hex
                .replace('#', "")
                .chars()
                .map(|c| c.to_string().repeat(2))
                .collect::<String>();
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
    pub fn argb_str(&self) -> String {
        Self::argb8_to_hex(self.argb())
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use argb_str()")]
    pub fn get_argb_str(&self) -> String {
        Self::argb8_to_hex(self.argb())
    }

    #[must_use]
    pub fn argb(&self) -> ARGB8 {
        if let Some(idx) = self.indexed {
            if let Some(v) = INDEX_TO_COLOR.get(&idx) {
                return *v;
            }
        }
        self.argb.unwrap_or_default()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use argb()")]
    pub fn get_argb(&self) -> ARGB8 {
        self.argb()
    }

    /// Get Argb.
    /// Color information based on the theme can also be obtained.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let theme = book.get_theme();
    /// ```
    #[must_use]
    pub fn argb_with_theme(&self, theme: &Theme) -> Cow<'static, str> {
        if self.indexed.is_some() {
            return self.argb_str().into();
        }
        if let Some(key) = self.theme_index {
            if let Some(v) = theme
                .theme_elements()
                .color_scheme()
                .color_map()
                .get(key as usize)
            {
                if let Some(tint) = self.tint {
                    return calc_tint(v, tint).into();
                }
                return v.clone().into();
            }
        }
        self.argb_str().clone().into()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use argb_with_theme()")]
    pub fn get_argb_with_theme(&self, theme: &Theme) -> Cow<'static, str> {
        self.argb_with_theme(theme)
    }

    pub fn set_argb<S: Into<ARGB8>>(&mut self, value: S) -> &mut Self {
        let argb = value.into();
        let indexed = COLOR_STR_TO_INDEX.get(Self::argb8_to_hex(argb).as_ref());

        if let Some(v) = indexed {
            self.indexed = Some(*v);
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
        let indexed = COLOR_STR_TO_INDEX.get(value.as_ref());

        if let Some(v) = indexed {
            self.indexed = Some(*v);
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
    pub fn indexed(&self) -> u32 {
        self.indexed.unwrap_or(0)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use indexed()")]
    pub fn get_indexed(&self) -> u32 {
        self.indexed()
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
    pub fn theme_index(&self) -> u32 {
        self.theme_index.unwrap_or(0)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use theme_index()")]
    pub fn get_theme_index(&self) -> u32 {
        self.theme_index()
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
    pub fn tint(&self) -> f64 {
        self.tint.unwrap_or(0.0)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tint()")]
    pub fn get_tint(&self) -> f64 {
        self.tint()
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
        assert_eq!(obj.argb_str(), "F34F8080");

        let mut obj = Color::default();
        obj.set_argb_str("FFFF8080");
        assert_eq!(obj.indexed(), 29);
        assert_eq!(obj.argb_str(), "FFFF8080");

        let mut obj = Color::default();
        let theme = Theme::default_value();
        obj.set_theme_index(1);
        assert_eq!(obj.argb_with_theme(&theme), "000000");
    }
}
