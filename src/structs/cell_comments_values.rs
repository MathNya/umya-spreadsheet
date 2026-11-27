use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum CellCommentsValues {
    AsDisplayed,
    AtEnd,
    None,
}
impl Default for CellCommentsValues {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for CellCommentsValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::AsDisplayed => "asDisplayed",
            Self::AtEnd => "atEnd",
            Self::None => "none",
        }
    }
}
impl FromStr for CellCommentsValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "asDisplayed" => Ok(Self::AsDisplayed),
            "atEnd" => Ok(Self::AtEnd),
            "none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}
