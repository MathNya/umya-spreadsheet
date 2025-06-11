use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TimeUnitValues {
    Days,
    Months,
    Years,
}
impl Default for TimeUnitValues {
    fn default() -> Self {
        Self::Days
    }
}
impl EnumTrait for TimeUnitValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Days => "days",
            Self::Months => "months",
            Self::Years => "years",
        }
    }
}
impl FromStr for TimeUnitValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "days" => Ok(Self::Days),
            "months" => Ok(Self::Months),
            "years" => Ok(Self::Years),
            _ => Err(()),
        }
    }
}
