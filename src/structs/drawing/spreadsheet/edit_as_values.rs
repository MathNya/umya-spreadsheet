use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum EditAsValues {
    Absolute,
    OneCell,
    TwoCell,
}
impl Default for EditAsValues {
    fn default() -> Self {
        Self::TwoCell
    }
}
impl EnumTrait for EditAsValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Absolute => "absolute",
            Self::OneCell => "oneCell",
            Self::TwoCell => "twoCell",
        }
    }
}
impl FromStr for EditAsValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "absolute" => Ok(Self::Absolute),
            "oneCell" => Ok(Self::OneCell),
            "twoCell" => Ok(Self::TwoCell),
            _ => Err(()),
        }
    }
}
