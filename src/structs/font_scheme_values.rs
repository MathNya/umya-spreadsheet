use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FontSchemeValues {
    Major,
    Minor,
    None,
}
impl Default for FontSchemeValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for FontSchemeValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::None => "none",
        }
    }
}
impl FromStr for FontSchemeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}
