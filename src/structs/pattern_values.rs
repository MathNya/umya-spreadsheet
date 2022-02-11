use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum PatternValues {
    DarkDown,
    DarkGray,
    DarkGrid,
    DarkHorizontal,
    DarkTrellis,
    DarkUp,
    DarkVertical,
    Gray0625,
    Gray125,
    LightDown,
    LightGray,
    LightGrid,
    LightHorizontal,
    LightTrellis,
    LightUp,
    LightVertical,
    MediumGray,
    None,
    Solid,
}
impl Default for PatternValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for PatternValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::DarkDown => "darkDown",
            Self::DarkGray => "darkGray",
            Self::DarkGrid => "darkGrid",
            Self::DarkHorizontal => "darkHorizontal",
            Self::DarkTrellis => "darkTrellis",
            Self::DarkUp => "darkUp",
            Self::DarkVertical => "darkVertical",
            Self::Gray0625 => "gray0625",
            Self::Gray125 => "gray125",
            Self::LightDown => "lightDown",
            Self::LightGray => "lightGray",
            Self::LightGrid => "lightGrid",
            Self::LightHorizontal => "lightHorizontal",
            Self::LightTrellis => "lightTrellis",
            Self::LightUp => "lightUp",
            Self::LightVertical => "lightVertical",
            Self::MediumGray => "mediumGray",
            Self::None => "none",
            Self::Solid => "solid",
        }
    }
}
impl FromStr for PatternValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "darkDown" => Ok(Self::DarkDown),
            "darkGray" => Ok(Self::DarkGray),
            "darkGrid" => Ok(Self::DarkGrid),
            "darkHorizontal" => Ok(Self::DarkHorizontal),
            "darkTrellis" => Ok(Self::DarkTrellis),
            "darkUp" => Ok(Self::DarkUp),
            "darkVertical" => Ok(Self::DarkVertical),
            "gray0625" => Ok(Self::Gray0625),
            "gray125" => Ok(Self::Gray125),
            "lightDown" => Ok(Self::LightDown),
            "lightGray" => Ok(Self::LightGray),
            "lightGrid" => Ok(Self::LightGrid),
            "lightHorizontal" => Ok(Self::LightHorizontal),
            "lightTrellis" => Ok(Self::LightTrellis),
            "lightUp" => Ok(Self::LightUp),
            "lightVertical" => Ok(Self::LightVertical),
            "mediumGray" => Ok(Self::MediumGray),
            "none" => Ok(Self::None),
            "solid" => Ok(Self::Solid),
            _ => Err(()),
        }
    }
}
