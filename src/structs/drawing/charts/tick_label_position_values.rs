use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TickLabelPositionValues {
    High,
    Low,
    NextTo,
    None,
}
impl Default for TickLabelPositionValues {
    fn default() -> Self {
        Self::High
    }
}
impl EnumTrait for TickLabelPositionValues {
    fn get_value_string(&self) -> &str {
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
