use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum TextFontAlignmentValues {
    Automatic,
    Baseline,
    Bottom,
    Center,
    Top,
}
impl Default for TextFontAlignmentValues {
    #[inline]
    fn default() -> Self {
        Self::Automatic
    }
}
impl EnumTrait for TextFontAlignmentValues {
    #[inline]
    fn value_string(&self) -> &str {
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

    #[inline]
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
