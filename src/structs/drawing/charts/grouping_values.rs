use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum GroupingValues {
    #[default]
    PercentStacked,
    Stacked,
    Standard,
}
impl EnumTrait for GroupingValues {
    fn value_string(&self) -> &str {
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
