use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TextCapsValues {
    All,
    None,
    Small,
}
impl Default for TextCapsValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TextCapsValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::All => "all",
            Self::None => "none",
            Self::Small => "small",
        }
    }
}
impl FromStr for TextCapsValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "small" => Ok(Self::Small),
            _ => Err(()),
        }
    }
}
