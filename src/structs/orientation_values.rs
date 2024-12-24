use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum OrientationValues {
    Default,
    Landscape,
    Portrait,
}
impl Default for OrientationValues {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}
impl EnumTrait for OrientationValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Default => "default",
            Self::Landscape => "landscape",
            Self::Portrait => "portrait",
        }
    }
}
impl FromStr for OrientationValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "default" => Ok(Self::Default),
            "landscape" => Ok(Self::Landscape),
            "portrait" => Ok(Self::Portrait),
            _ => Err(()),
        }
    }
}
