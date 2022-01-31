use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum PaneValues {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
impl Default for PaneValues {
    fn default() -> Self {
        Self::BottomRight
    }
}
impl EnumTrait for PaneValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::BottomLeft => "bottomLeft",
            Self::BottomRight => "bottomRight",
            Self::TopLeft => "topLeft",
            Self::TopRight => "TopRight",
        }
    }
}
impl FromStr for PaneValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bottomLeft" => Ok(Self::BottomLeft),
            "bottomRight" => Ok(Self::BottomRight),
            "topLeft" => Ok(Self::TopLeft),
            "TopRight" => Ok(Self::TopRight),
            _ => Err(()),
        }
    }
}
