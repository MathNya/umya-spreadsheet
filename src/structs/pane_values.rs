use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PaneValues {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
impl Default for PaneValues {
    #[inline]
    fn default() -> Self {
        Self::BottomRight
    }
}
impl EnumTrait for PaneValues {
    #[inline]
    fn value_string(&self) -> &str {
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

    #[inline]
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
