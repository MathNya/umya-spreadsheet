use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalculateModeValues {
    Auto,
    AutoNoTable,
    Manual,
}
impl Default for CalculateModeValues {
    #[inline]
    fn default() -> Self {
        Self::Auto
    }
}
impl EnumTrait for CalculateModeValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Auto => "auto",
            Self::AutoNoTable => "autoNoTable",
            Self::Manual => "manual",
        }
    }
}
impl FromStr for CalculateModeValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "auto" => Ok(Self::Auto),
            "autoNoTable" => Ok(Self::AutoNoTable),
            "manual" => Ok(Self::Manual),
            _ => Err(()),
        }
    }
}
