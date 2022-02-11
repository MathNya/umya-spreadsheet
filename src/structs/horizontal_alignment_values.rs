use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum HorizontalAlignmentValues {
    Center,
    CenterContinuous,
    Distributed,
    Fill,
    General,
    Justify,
    Left,
    Right,
}
impl Default for HorizontalAlignmentValues {
    fn default() -> Self {
        Self::General
    }
}
impl EnumTrait for HorizontalAlignmentValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Center => "center",
            Self::CenterContinuous => "centerContinuous",
            Self::Distributed => "distributed",
            Self::Fill => "fill",
            Self::General => "general",
            Self::Justify => "justify",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
impl FromStr for HorizontalAlignmentValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "center" => Ok(Self::Center),
            "centerContinuous" => Ok(Self::CenterContinuous),
            "distributed" => Ok(Self::Distributed),
            "fill" => Ok(Self::Fill),
            "general" => Ok(Self::General),
            "justify" => Ok(Self::Justify),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
