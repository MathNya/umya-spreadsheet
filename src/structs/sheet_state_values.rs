use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum SheetStateValues {
    Hidden,
    VeryHidden,
    Visible,
}
impl Default for SheetStateValues {
    #[inline]
    fn default() -> Self {
        Self::Visible
    }
}
impl EnumTrait for SheetStateValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Hidden => "hidden",
            Self::VeryHidden => "veryHidden",
            Self::Visible => "visible",
        }
    }
}
impl FromStr for SheetStateValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "hidden" => Ok(Self::Hidden),
            "veryHidden" => Ok(Self::VeryHidden),
            "visible" => Ok(Self::Visible),
            _ => Err(()),
        }
    }
}
