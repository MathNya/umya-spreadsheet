use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum TextCapsValues {
    All,
    None,
    Small,
}
impl Default for TextCapsValues {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TextCapsValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::All => "all",
            Self::None => "none",
            Self::Small => "small",
        }
    }
}
impl FromStr for TextCapsValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "small" => Ok(Self::Small),
            _ => Err(()),
        }
    }
}
