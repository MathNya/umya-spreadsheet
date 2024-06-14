use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TextFontAlignmentValues {
    Automatic,
    Baseline,
    Bottom,
    Center,
    Top,
}
impl Default for TextFontAlignmentValues {
    fn default() -> Self {
        Self::Automatic
    }
}
impl EnumTrait for TextFontAlignmentValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Automatic => "auto",
            Self::Baseline => "base",
            Self::Bottom => "b",
            Self::Center => "ctr",
            Self::Top => "t",
        }
    }
}
impl FromStr for TextFontAlignmentValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "auto" => Ok(Self::Automatic),
            "base" => Ok(Self::Baseline),
            "b" => Ok(Self::Bottom),
            "ctr" => Ok(Self::Center),
            "t" => Ok(Self::Top),
            _ => Err(()),
        }
    }
}
