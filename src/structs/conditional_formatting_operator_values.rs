use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConditionalFormattingOperatorValues {
    BeginsWith,
    Between,
    ContainsText,
    EndsWith,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotBetween,
    NotContains,
    NotEqual,
}
impl Default for ConditionalFormattingOperatorValues {
    fn default() -> Self {
        Self::LessThan
    }
}
impl EnumTrait for ConditionalFormattingOperatorValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::BeginsWith => "beginsWith",
            Self::Between => "between",
            Self::ContainsText => "containsText",
            Self::EndsWith => "endsWith",
            Self::Equal => "equal",
            Self::GreaterThan => "greaterThan",
            Self::GreaterThanOrEqual => "greaterThanOrEqual",
            Self::LessThan => "lessThan",
            Self::LessThanOrEqual => "lessThanOrEqual",
            Self::NotBetween => "notBetween",
            Self::NotContains => "notContains",
            Self::NotEqual => "notEqual",
        }
    }
}
impl FromStr for ConditionalFormattingOperatorValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "beginsWith" => Ok(Self::BeginsWith),
            "between" => Ok(Self::Between),
            "containsText" => Ok(Self::ContainsText),
            "endsWith" => Ok(Self::EndsWith),
            "equal" => Ok(Self::Equal),
            "greaterThan" => Ok(Self::GreaterThan),
            "greaterThanOrEqual" => Ok(Self::GreaterThanOrEqual),
            "lessThan" => Ok(Self::LessThan),
            "lessThanOrEqual" => Ok(Self::LessThanOrEqual),
            "notBetween" => Ok(Self::NotBetween),
            "notContains" => Ok(Self::NotContains),
            "notEqual" => Ok(Self::NotEqual),
            _ => Err(()),
        }
    }
}
