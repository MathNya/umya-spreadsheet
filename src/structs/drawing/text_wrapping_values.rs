use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TextWrappingValues {
    None,
    Square,
}
impl Default for TextWrappingValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TextWrappingValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::None => "none",
            Self::Square => "square",
        }
    }
}
impl FromStr for TextWrappingValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "none" => Ok(Self::None),
            "square" => Ok(Self::Square),
            _ => Err(()),
        }
    }
}
