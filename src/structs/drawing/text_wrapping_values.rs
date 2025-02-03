use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum TextWrappingValues {
    None,
    Square,
}
impl Default for TextWrappingValues {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TextWrappingValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::None => "none",
            Self::Square => "square",
        }
    }
}
impl FromStr for TextWrappingValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "none" => Ok(Self::None),
            "square" => Ok(Self::Square),
            _ => Err(()),
        }
    }
}
