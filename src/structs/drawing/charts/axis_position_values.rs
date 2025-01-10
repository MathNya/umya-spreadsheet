use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum AxisPositionValues {
    Bottom,
    Left,
    Right,
    Top,
}
impl Default for AxisPositionValues {
    fn default() -> Self {
        Self::Bottom
    }
}
impl EnumTrait for AxisPositionValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bottom => "b",
            Self::Left => "l",
            Self::Right => "r",
            Self::Top => "t",
        }
    }
}
impl FromStr for AxisPositionValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "b" => Ok(Self::Bottom),
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            "t" => Ok(Self::Top),
            _ => Err(()),
        }
    }
}
