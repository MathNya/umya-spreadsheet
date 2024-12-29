use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum TotalsRowFunctionValues {
    Average,
    Count,
    CountNumbers,
    Custom,
    Maximum,
    Minimum,
    None,
    StandardDeviation,
    Sum,
    Variance,
}
impl Default for TotalsRowFunctionValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for TotalsRowFunctionValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Average => "average",
            Self::Count => "count",
            Self::CountNumbers => "countNums",
            Self::Custom => "custom",
            Self::Maximum => "max",
            Self::Minimum => "min",
            Self::None => "none",
            Self::StandardDeviation => "stdDev",
            Self::Sum => "sum",
            Self::Variance => "var",
        }
    }
}
impl FromStr for TotalsRowFunctionValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "average" => Ok(Self::Average),
            "count" => Ok(Self::Count),
            "countNums" => Ok(Self::CountNumbers),
            "custom" => Ok(Self::Custom),
            "max" => Ok(Self::Maximum),
            "min" => Ok(Self::Minimum),
            "none" => Ok(Self::None),
            "stdDev" => Ok(Self::StandardDeviation),
            "sum" => Ok(Self::Sum),
            "var" => Ok(Self::Variance),
            _ => Err(()),
        }
    }
}
