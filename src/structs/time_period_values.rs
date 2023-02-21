use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TimePeriodValues {
    Last7Days,
    LastMonth,
    LastWeek,
    NextMonth,
    NextWeek,
    ThisMonth,
    ThisWeek,
    Today,
    Tomorrow,
    Yesterday,
}
impl Default for TimePeriodValues {
    fn default() -> Self {
        Self::Today
    }
}
impl EnumTrait for TimePeriodValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Last7Days => "last7Days",
            Self::LastMonth => "lastMonth",
            Self::LastWeek => "lastWeek",
            Self::NextMonth => "nextMonth",
            Self::NextWeek => "nextWeek",
            Self::ThisMonth => "thisMonth",
            Self::ThisWeek => "thisWeek",
            Self::Today => "today",
            Self::Tomorrow => "tomorrow",
            Self::Yesterday => "yesterday",
        }
    }
}
impl FromStr for TimePeriodValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "last7Days" => Ok(Self::Last7Days),
            "lastMonth" => Ok(Self::LastMonth),
            "lastWeek" => Ok(Self::LastWeek),
            "nextMonth" => Ok(Self::NextMonth),
            "nextWeek" => Ok(Self::NextWeek),
            "thisMonth" => Ok(Self::ThisMonth),
            "thisWeek" => Ok(Self::ThisWeek),
            "today" => Ok(Self::Today),
            "tomorrow" => Ok(Self::Tomorrow),
            "yesterday" => Ok(Self::Yesterday),
            _ => Err(()),
        }
    }
}
