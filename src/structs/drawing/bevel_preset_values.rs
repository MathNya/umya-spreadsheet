use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum BevelPresetValues {
    Angle,
    ArtDeco,
    Circle,
    Convex,
    CoolSlant,
    Cross,
    Divot,
    HardEdge,
    RelaxedInset,
    Riblet,
    Slope,
    SoftRound,
}
impl Default for BevelPresetValues {
    fn default() -> Self {
        Self::RelaxedInset
    }
}
impl EnumTrait for BevelPresetValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Angle => "angle",
            Self::ArtDeco => "artDeco",
            Self::Circle => "circle",
            Self::Convex => "convex",
            Self::CoolSlant => "coolSlant",
            Self::Cross => "cross",
            Self::Divot => "divot",
            Self::HardEdge => "hardEdge",
            Self::RelaxedInset => "relaxedInset",
            Self::Riblet => "riblet",
            Self::Slope => "slope",
            Self::SoftRound => "softRound",
        }
    }
}
impl FromStr for BevelPresetValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "angle" => Ok(Self::Angle),
            "artDeco" => Ok(Self::ArtDeco),
            "circle" => Ok(Self::Circle),
            "convex" => Ok(Self::Convex),
            "coolSlant" => Ok(Self::CoolSlant),
            "cross" => Ok(Self::Cross),
            "divot" => Ok(Self::Divot),
            "hardEdge" => Ok(Self::HardEdge),
            "relaxedInset" => Ok(Self::RelaxedInset),
            "riblet" => Ok(Self::Riblet),
            "slope" => Ok(Self::Slope),
            "softRound" => Ok(Self::SoftRound),
            _ => Err(()),
        }
    }
}
