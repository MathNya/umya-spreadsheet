use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum ScatterStyleValues {
    #[default]
    Line,
    LineMarker,
    Marker,
    Smooth,
    SmoothMarker,
}
impl EnumTrait for ScatterStyleValues {
    fn value_string(&self) -> &str {
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
