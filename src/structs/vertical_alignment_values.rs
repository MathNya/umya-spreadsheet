use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum VerticalAlignmentValues {
    Bottom,
    Center,
    Distributed,
    Justify,
    Top,
}
impl Default for VerticalAlignmentValues {
    fn default() -> Self {
        Self::Top
    }
}
impl EnumTrait for VerticalAlignmentValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bottom => "bottom",
            Self::Center => "center",
            Self::Distributed => "distributed",
            Self::Justify => "justify",
            Self::Top => "top",
        }
    }
}
impl FromStr for VerticalAlignmentValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bottom" => Ok(Self::Bottom),
            "center" => Ok(Self::Center),
            "distributed" => Ok(Self::Distributed),
            "justify" => Ok(Self::Justify),
            "top" => Ok(Self::Top),
            _ => Err(()),
        }
    }
}
