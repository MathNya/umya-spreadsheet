use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConditionalFormatValueObjectValues {
    Formula,
    Max,
    Min,
    Number,
    Percent,
    Percentile,
}
impl Default for ConditionalFormatValueObjectValues {
    fn default() -> Self {
        Self::Number
    }
}
impl EnumTrait for ConditionalFormatValueObjectValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Formula => "formula",
            Self::Max => "max",
            Self::Min => "min",
            Self::Number => "num",
            Self::Percent => "percent",
            Self::Percentile => "percentile",
        }
    }
}
impl FromStr for ConditionalFormatValueObjectValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "formula" => Ok(Self::Formula),
            "max" => Ok(Self::Max),
            "min" => Ok(Self::Min),
            "num" => Ok(Self::Number),
            "percent" => Ok(Self::Percent),
            "percentile" => Ok(Self::Percentile),
            _ => Err(()),
        }
    }
}
