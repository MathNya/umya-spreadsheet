use std::io::Cursor;

use md5::Digest;
use phf::phf_map;
use quick_xml::{
    Reader,
    Writer,
    escape,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NumberingFormat {
    number_format_id: u32,
    format_code:      Box<str>,
    is_build_in:      bool,
}

impl Default for NumberingFormat {
    #[inline]
    fn default() -> Self {
        Self {
            number_format_id: 0,
            format_code:      NumberingFormat::FORMAT_GENERAL.into(),
            is_build_in:      true,
        }
    }
}

impl NumberingFormat {
    pub const FORMAT_ACCOUNTING_EUR: &'static str =
        r#"_("€"* #,##0.00_);_("€"* \(#,##0.00\);_("€"* "-"??_);_(@_)"#;
    pub const FORMAT_ACCOUNTING_USD: &'static str =
        r#"_("$"* #,##0.00_);_("$"* \(#,##0.00\);_("$"* "-"??_);_(@_)"#;
    pub const FORMAT_CURRENCY_EUR: &'static str = r#"#,##0_-"€""#;
    pub const FORMAT_CURRENCY_EUR_SIMPLE: &'static str = r#"#,##0.00_-"€""#;
    pub const FORMAT_CURRENCY_USD: &'static str = r"$#,##0_-";
    pub const FORMAT_CURRENCY_USD_SIMPLE: &'static str = r##""$"#,##0.00_-"##;
    pub const FORMAT_DATE_DATETIME: &'static str = "d/m/yy h:mm";
    pub const FORMAT_DATE_DDMMYYYY: &'static str = "dd-mm-yyyy";
    pub const FORMAT_DATE_DDMMYYYYSLASH: &'static str = "dd/mm/yyyy";
    pub const FORMAT_DATE_DMMINUS: &'static str = "d-m";
    pub const FORMAT_DATE_DMYMINUS: &'static str = "d-m-yy";
    pub const FORMAT_DATE_DMYSLASH: &'static str = "d/m/yy";
    pub const FORMAT_DATE_MYMINUS: &'static str = "m-yy";
    pub const FORMAT_DATE_TIME1: &'static str = "h:mm AM/PM";
    pub const FORMAT_DATE_TIME2: &'static str = "h:mm:ss AM/PM";
    pub const FORMAT_DATE_TIME3: &'static str = "h:mm";
    pub const FORMAT_DATE_TIME4: &'static str = "h:mm:ss";
    pub const FORMAT_DATE_TIME5: &'static str = "mm:ss";
    pub const FORMAT_DATE_TIME6: &'static str = "h:mm:ss";
    pub const FORMAT_DATE_TIME8: &'static str = "h:mm:ss;@";
    pub const FORMAT_DATE_XLSX14: &'static str = "mm-dd-yy";
    pub const FORMAT_DATE_XLSX15: &'static str = "d-mmm-yy";
    pub const FORMAT_DATE_XLSX16: &'static str = "d-mmm";
    pub const FORMAT_DATE_XLSX17: &'static str = "mmm-yy";
    pub const FORMAT_DATE_XLSX22: &'static str = "m/d/yy h:mm";
    pub const FORMAT_DATE_YYYYMMDD: &'static str = "yyyy-mm-dd";
    pub const FORMAT_DATE_YYYYMMDD2: &'static str = "yyyy-mm-dd";
    pub const FORMAT_DATE_YYYYMMDDSLASH: &'static str = "yyyy/mm/dd;@";
    // Pre-defined formats
    pub const FORMAT_GENERAL: &'static str = "General";
    pub const FORMAT_NUMBER: &'static str = "0";
    pub const FORMAT_NUMBER_00: &'static str = "0.00";
    pub const FORMAT_NUMBER_COMMA_SEPARATED1: &'static str = "#,##0.00";
    pub const FORMAT_NUMBER_COMMA_SEPARATED2: &'static str = "#,##0.00_-";
    pub const FORMAT_PERCENTAGE: &'static str = "0%";
    pub const FORMAT_PERCENTAGE_00: &'static str = "0.00%";
    pub const FORMAT_TEXT: &'static str = "@";

    #[inline]
    #[must_use]
    pub fn get_number_format_id(&self) -> u32 {
        self.number_format_id
    }

    pub fn set_number_format_id(&mut self, value: u32) -> &mut Self {
        let format_code_result = FILL_BUILT_IN_FORMAT_CODES.entries().find_map(|(key, val)| {
            if key == &value {
                Some(val.to_owned())
            } else {
                None
            }
        });

        self.format_code = format_code_result
            .expect("Not Found NumberFormatId.")
            .to_owned()
            .into_boxed_str();
        self.number_format_id = value;
        self.is_build_in = true;
        self
    }

    #[inline]
    pub(crate) fn set_number_format_id_crate(&mut self, value: u32) -> &mut Self {
        self.number_format_id = value;
        self
    }

    /// Set the format code.
    /// # Arguments
    /// * `value` - format code. (`umya_spreadsheet::NumberingFormat`)
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0).unwrap();
    /// let _unused = worksheet
    ///     .get_style_mut("C30")
    ///     .get_number_format_mut()
    ///     .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_DATE_XLSX17);
    /// ```
    pub fn set_format_code<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.format_code = value.into().into_boxed_str();
        for (index, format) in FILL_BUILT_IN_FORMAT_CODES.entries() {
            if &&*self.format_code == format {
                self.number_format_id = *index;
                self.is_build_in = true;
                return self;
            }
        }
        self.number_format_id = 999_999;
        self.is_build_in = false;
        self
    }

    #[inline]
    pub(crate) fn set_format_code_crate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.format_code = value.into().into_boxed_str();
        self
    }

    #[inline]
    #[must_use]
    pub fn get_format_code(&self) -> &str {
        &self.format_code
    }

    #[inline]
    pub(crate) fn get_is_build_in(&self) -> bool {
        self.is_build_in
    }

    #[inline]
    pub(crate) fn get_hash_code(&self) -> String {
        format!("{:x}", md5::Md5::digest(&*self.format_code))
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.number_format_id = get_attribute(e, b"numFmtId")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        self.format_code = escape::unescape(get_attribute(e, b"formatCode").unwrap().as_str())
            .unwrap()
            .to_string()
            .into_boxed_str();
        self.is_build_in = false;
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, number_format_id: u32) {
        // numFmt
        write_start_tag(
            writer,
            "numFmt",
            vec![
                ("numFmtId", number_format_id.to_string().as_str()),
                ("formatCode", &self.format_code),
            ],
            true,
        );
    }
}

pub(crate) static FILL_BUILT_IN_FORMAT_CODES: phf::Map<u32, &'static str> = phf_map! {
    0u32 => NumberingFormat::FORMAT_GENERAL,
    1u32 => "0",
    2u32 => "0.00",
    3u32 => "#,##0",
    4u32 => "#,##0.00",

    9u32 => "0%",
    10u32 => "0.00%",
    11u32 => "0.00E+00",
    12u32 => "# ?/?",
    13u32 => "# ??/??",
    14u32 => "m/d/yyyy",
    15u32 => "d-mmm-yy",
    16u32 => "d-mmm",
    17u32 => "mmm-yy",
    18u32 => "h:mm AM/PM",
    19u32 => "h:mm:ss AM/PM",
    20u32 => "h:mm",
    21u32 => "h:mm:ss",
    22u32 => "m/d/yyyy h:mm",

    37u32 => "#,##0_);(#,##0)",
    38u32 => "#,##0_);[Red](#,##0)",
    39u32 => "#,##0.00_);(#,##0.00)",
    40u32 => "#,##0.00_);[Red](#,##0.00)",

    44u32 => r#"_("$"* #,##0.00_);_("$"* \(#,##0.00\);_("$"* "-"??_);_(@_)"#,
    45u32 => "mm:ss",
    46u32 => "[h]:mm:ss",
    47u32 => "mm:ss.0",
    48u32 => "##0.0E+0",
    49u32 => "@",

    // CHT
    27u32 => "[$-404]e/m/d",
    30u32 => "m/d/yy",
    36u32 => "[$-404]e/m/d",
    50u32 => "[$-404]e/m/d",
    57u32 => "[$-404]e/m/d",

    // THA
    59u32 => "t0",
    60u32 => "t0.00",
    61u32 => "t#,##0",
    62u32 => "t#,##0.00",
    67u32 => "t0%",
    68u32 => "t0.00%",
    69u32 => "t# ?/?",
    70u32 => "t# ??/??",

    // JPN
    28u32 => r#"[$-411]ggge"年"m"月"d"日""#,
    29u32 => r#"[$-411]ggge"年"m"月"d"日""#,
    31u32 => r#"yyyy"年"m"月"d"日""#,
    32u32 => r#"h"時"mm"分""#,
    33u32 => r#"h"時"mm"分"ss"秒""#,
    34u32 => r#"yyyy"年"m"月""#,
    35u32 => r#"m"月"d"日""#,
    51u32 => r#"[$-411]ggge"年"m"月"d"日""#,
    52u32 => r#"yyyy"年"m"月""#,
    53u32 => r#"m"月"d"日""#,
    54u32 => r#"[$-411]ggge"年"m"月"d"日""#,
    55u32 => r#"yyyy"年"m"月""#,
    56u32 => r#"m"月"d"日""#,
    58u32 => r#"[$-411]ggge"年"m"月"d"日""#,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_number_format_id() {
        let mut obj = NumberingFormat::default();

        obj.set_number_format_id(0);
        assert_eq!(obj.get_format_code(), "General");

        obj.set_number_format_id(1);
        assert_eq!(obj.get_format_code(), "0");
    }
}
