use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum RadarStyleValues {
    Filled,
    Marker,
    Standard,
}
impl Default for RadarStyleValues {
    fn default() -> Self {
        Self::Standard
    }
}
impl EnumTrait for RadarStyleValues {
    fn value_string(&self) -> &str {
        match &self {
            Self::Filled => "filled",
            Self::Marker => "marker",
            Self::Standard => "standard",
        }
    }
}
impl FromStr for RadarStyleValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "filled" => Ok(Self::Filled),
            "marker" => Ok(Self::Marker),
            "standard" => Ok(Self::Standard),
            _ => Err(()),
        }
    }
}
