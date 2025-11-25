use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum ShapeValues {
    Box,
    #[default]
    Cone,
    ConeToMax,
    Cylinder,
    Pyramid,
    PyramidToMaximum,
}
impl EnumTrait for ShapeValues {
    fn value_string(&self) -> &str {
        match &self {
            Self::Box => "box",
            Self::Cone => "cone",
            Self::ConeToMax => "coneToMax",
            Self::Cylinder => "cylinder",
            Self::Pyramid => "pyramid",
            Self::PyramidToMaximum => "pyramidToMax",
        }
    }
}
impl FromStr for ShapeValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "box" => Ok(Self::Box),
            "cone" => Ok(Self::Cone),
            "coneToMax" => Ok(Self::ConeToMax),
            "cylinder" => Ok(Self::Cylinder),
            "pyramid" => Ok(Self::Pyramid),
            "pyramidToMax" => Ok(Self::PyramidToMaximum),
            _ => Err(()),
        }
    }
}
