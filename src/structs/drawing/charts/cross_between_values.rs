use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum CrossBetweenValues {
    Between,
    MidpointCategory,
}
impl Default for CrossBetweenValues {
    fn default() -> Self {
        Self::Between
    }
}
impl EnumTrait for CrossBetweenValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Between => "between",
            Self::MidpointCategory => "midCat",
        }
    }
}
impl FromStr for CrossBetweenValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "between" => Ok(Self::Between),
            "midCat" => Ok(Self::MidpointCategory),
            _ => Err(()),
        }
    }
}
