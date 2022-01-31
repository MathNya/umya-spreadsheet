use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TileFlipValues {
    Horizontal,
    HorizontalAndVertical,
    None,
    Vertical,
}
impl Default for TileFlipValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TileFlipValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Horizontal => "x",
            Self::HorizontalAndVertical => "xy",
            Self::None => "none",
            Self::Vertical => "y",
        }
    }
}
impl FromStr for TileFlipValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x" => Ok(Self::Horizontal),
            "xy" => Ok(Self::HorizontalAndVertical),
            "none" => Ok(Self::None),
            "y" => Ok(Self::Vertical),
            _ => Err(()),
        }
    }
}
