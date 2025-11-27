use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum TickLabelPositionValues {
    #[default]
    High,
    Low,
    NextTo,
    None,
}
impl EnumTrait for TickLabelPositionValues {
    fn value_string(&self) -> &str {
        match &self {
            Self::High => "high",
            Self::Low => "low",
            Self::NextTo => "nextTo",
            Self::None => "none",
        }
    }
}
impl FromStr for TickLabelPositionValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "nextTo" => Ok(Self::NextTo),
            "none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}
