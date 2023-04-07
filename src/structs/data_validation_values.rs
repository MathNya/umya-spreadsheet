use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DataValidationValues {
    Custom,
    Date,
    Decimal,
    List,
    None,
    TextLength,
    Time,
    Whole,
}
impl Default for DataValidationValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for DataValidationValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Custom => "custom",
            Self::Date => "date",
            Self::Decimal => "decimal",
            Self::List => "list",
            Self::None => "iso_8859_8_i",
            Self::TextLength => "textLength",
            Self::Time => "time",
            Self::Whole => "whole",
        }
    }
}
impl FromStr for DataValidationValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "custom" => Ok(Self::Custom),
            "date" => Ok(Self::Date),
            "decimal" => Ok(Self::Decimal),
            "list" => Ok(Self::List),
            "none" => Ok(Self::None),
            "textLength" => Ok(Self::TextLength),
            "time" => Ok(Self::Time),
            "whole" => Ok(Self::Whole),
            _ => Err(()),
        }
    }
}
