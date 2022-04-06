use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CsvEncodeValues {
    Utf8,
    ShiftJis,
    Koi8u,
    Koi8r,
    Iso88598i,
    Gbk,
    EucKr,
    Big5,
    Utf16Le,
    Utf16Be,
}
impl Default for CsvEncodeValues {
    fn default() -> Self {
        Self::Utf8
    }
}
impl EnumTrait for CsvEncodeValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Utf8 => "utf_8",
            Self::ShiftJis => "shift_jis",
            Self::Koi8u => "koi_8_u",
            Self::Koi8r => "koi_8_r",
            Self::Iso88598i => "iso_8859_8_i",
            Self::Gbk => "gbk",
            Self::EucKr => "euc_kr",
            Self::Big5 => "big_5",
            Self::Utf16Le => "utf_16_le",
            Self::Utf16Be => "utf_16_be",
        }
    }
}
impl FromStr for CsvEncodeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "utf_8" => Ok(Self::Utf8),
            "shift_jis" => Ok(Self::ShiftJis),
            "koi_8_u" => Ok(Self::Koi8u),
            "koi_8_r" => Ok(Self::Koi8r),
            "iso_8859_8_i" => Ok(Self::Iso88598i),
            "gbk" => Ok(Self::Gbk),
            "euc_kr" => Ok(Self::EucKr),
            "big_5" => Ok(Self::Big5),
            "utf_16_le" => Ok(Self::Utf16Le),
            "utf_16_be" => Ok(Self::Utf16Be),
            _ => Err(()),
        }
    }
}
