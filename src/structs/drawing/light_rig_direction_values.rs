use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LightRigDirectionValues {
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,
}
impl Default for LightRigDirectionValues {
    fn default() -> Self {
        Self::TopLeft
    }
}
impl EnumTrait for LightRigDirectionValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bottom => "b",
            Self::BottomLeft => "bl",
            Self::BottomRight => "br",
            Self::Left => "l",
            Self::Right => "r",
            Self::Top => "t",
            Self::TopLeft => "tl",
            Self::TopRight => "tr",
        }
    }
}
impl FromStr for LightRigDirectionValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "b" => Ok(Self::Bottom),
            "bl" => Ok(Self::BottomLeft),
            "br" => Ok(Self::BottomRight),
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            "t" => Ok(Self::Top),
            "tl" => Ok(Self::TopLeft),
            "tr" => Ok(Self::TopRight),
            _ => Err(()),
        }
    }
}
