use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum GroupingValues {
    PercentStacked,
    Stacked,
    Standard,
}
impl Default for GroupingValues {
    fn default() -> Self {
        Self::PercentStacked
    }
}
impl EnumTrait for GroupingValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::PercentStacked => "percentStacked",
            Self::Stacked => "stacked",
            Self::Standard => "standard",
        }
    }
}
impl FromStr for GroupingValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "percentStacked" => Ok(Self::PercentStacked),
            "stacked" => Ok(Self::Stacked),
            "standard" => Ok(Self::Standard),
            _ => Err(()),
        }
    }
}
