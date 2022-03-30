use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CsvEncodeValues {
    Utf8,
    ShiftJis,
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
        }
    }
}
impl FromStr for CsvEncodeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "utf_8" => Ok(Self::Utf8),
            "shift_jis" => Ok(Self::ShiftJis),
            _ => Err(()),
        }
    }
}
