use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ItemValues {
    Average,
    Blank,
    Count,
    CountA,
    Data,
    Default,
    Grand,
    Maximum,
    Minimum,
    Product,
    StandardDeviation,
    StandardDeviationP,
    Sum,
    Variance,
    VarianceP,
}
impl Default for ItemValues {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}
impl EnumTrait for ItemValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Average => "avg",
            Self::Blank => "blank",
            Self::Count => "count",
            Self::CountA => "countA",
            Self::Data => "data",
            Self::Default => "default",
            Self::Grand => "grand",
            Self::Maximum => "max",
            Self::Minimum => "min",
            Self::Product => "product",
            Self::StandardDeviation => "stdDev",
            Self::StandardDeviationP => "stdDevP",
            Self::Sum => "sum",
            Self::Variance => "var",
            Self::VarianceP => "varP",
        }
    }
}
impl FromStr for ItemValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "avg" => Ok(Self::Average),
            "blank" => Ok(Self::Blank),
            "count" => Ok(Self::Count),
            "countA" => Ok(Self::CountA),
            "data" => Ok(Self::Data),
            "default" => Ok(Self::Default),
            "grand" => Ok(Self::Grand),
            "max" => Ok(Self::Maximum),
            "min" => Ok(Self::Minimum),
            "product" => Ok(Self::Product),
            "stdDev" => Ok(Self::StandardDeviation),
            "stdDevP" => Ok(Self::StandardDeviationP),
            "sum" => Ok(Self::Sum),
            "var" => Ok(Self::Variance),
            "varP" => Ok(Self::VarianceP),
            _ => Err(()),
        }
    }
}
