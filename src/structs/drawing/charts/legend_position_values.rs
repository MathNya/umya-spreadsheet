use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LegendPositionValues {
    Bottom,
    Left,
    Right,
    Top,
    TopRight,
}
impl Default for LegendPositionValues {
    fn default() -> Self {
        Self::Bottom
    }
}
impl EnumTrait for LegendPositionValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bottom => "b",
            Self::Left => "l",
            Self::Right => "r",
            Self::Top => "t",
            Self::TopRight => "tr",
        }
    }
}
impl FromStr for LegendPositionValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "b" => Ok(Self::Bottom),
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            "t" => Ok(Self::Top),
            "tr" => Ok(Self::TopRight),
            _ => Err(()),
        }
    }
}
