use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum BorderStyleValues {
    DashDot,
    DashDotDot,
    Dashed,
    Dotted,
    Double,
    Hair,
    Medium,
    MediumDashDot,
    MediumDashDotDot,
    MediumDashed,
    None,
    SlantDashDot,
    Thick,
    Thin,
}
impl Default for BorderStyleValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for BorderStyleValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::DashDot => "dashDot",
            Self::DashDotDot => "dashDotDot",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::Hair => "hair",
            Self::Medium => "medium",
            Self::MediumDashDot => "mediumDashDot",
            Self::MediumDashDotDot => "mediumDashDotDot",
            Self::MediumDashed => "mediumDashed",
            Self::None => "none",
            Self::SlantDashDot => "slantDashDot",
            Self::Thick => "thick",
            Self::Thin => "thin",
        }
    }
}
impl FromStr for BorderStyleValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dashDot" => Ok(Self::DashDot),
            "dashDotDot" => Ok(Self::DashDotDot),
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "double" => Ok(Self::Double),
            "hair" => Ok(Self::Hair),
            "medium" => Ok(Self::Medium),
            "mediumDashDot" => Ok(Self::MediumDashDot),
            "mediumDashDotDot" => Ok(Self::MediumDashDotDot),
            "mediumDashed" => Ok(Self::MediumDashed),
            "none" => Ok(Self::None),
            "slantDashDot" => Ok(Self::SlantDashDot),
            "thick" => Ok(Self::Thick),
            "thin" => Ok(Self::Thin),
            _ => Err(()),
        }
    }
}
