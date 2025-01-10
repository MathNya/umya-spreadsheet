use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum OrientationValues {
    MaxMin,
    MinMax,
}
impl Default for OrientationValues {
    fn default() -> Self {
        Self::MaxMin
    }
}
impl EnumTrait for OrientationValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::MaxMin => "maxMin",
            Self::MinMax => "minMax",
        }
    }
}
impl FromStr for OrientationValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "maxMin" => Ok(Self::MaxMin),
            "minMax" => Ok(Self::MinMax),
            _ => Err(()),
        }
    }
}
