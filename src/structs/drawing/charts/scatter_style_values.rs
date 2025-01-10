use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum ScatterStyleValues {
    Line,
    LineMarker,
    Marker,
    Smooth,
    SmoothMarker,
}
impl Default for ScatterStyleValues {
    fn default() -> Self {
        Self::Line
    }
}
impl EnumTrait for ScatterStyleValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Line => "line",
            Self::LineMarker => "lineMarker",
            Self::Marker => "marker",
            Self::Smooth => "smooth",
            Self::SmoothMarker => "smoothMarker",
        }
    }
}
impl FromStr for ScatterStyleValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "line" => Ok(Self::Line),
            "lineMarker" => Ok(Self::LineMarker),
            "marker" => Ok(Self::Marker),
            "smooth" => Ok(Self::Smooth),
            "smoothMarker" => Ok(Self::SmoothMarker),
            _ => Err(()),
        }
    }
}
