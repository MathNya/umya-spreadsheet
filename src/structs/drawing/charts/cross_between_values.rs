use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum CrossBetweenValues {
    #[default]
    Between,
    MidpointCategory,
}
impl EnumTrait for CrossBetweenValues {
    fn value_string(&self) -> &str {
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
