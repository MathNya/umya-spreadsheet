use std::str::FromStr;

use super::EnumTrait;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DataValidationOperatorValues {
    Between,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotBetween,
    NotEqual,
}

impl EnumTrait for DataValidationOperatorValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match self {
            Self::Between => "between",
            Self::Equal => "equal",
            Self::GreaterThan => "greaterThan",
            Self::GreaterThanOrEqual => "greaterThanOrEqual",
            Self::LessThan => "lessThan",
            Self::LessThanOrEqual => "lessThanOrEqual",
            Self::NotBetween => "notBetween",
            Self::NotEqual => "notEqual",
        }
    }
}

impl Default for DataValidationOperatorValues {
    #[inline]
    fn default() -> Self {
        Self::LessThan
    }
}

impl FromStr for DataValidationOperatorValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "between" => Self::Between,
            "equal" => Self::Equal,
            "greaterThan" => Self::GreaterThan,
            "greaterThanOrEqual" => Self::GreaterThanOrEqual,
            "lessThan" => Self::LessThan,
            "lessThanOrEqual" => Self::LessThanOrEqual,
            "notBetween" => Self::NotBetween,
            "notEqual" => Self::NotEqual,
            _ => return Err(()),
        })
    }
}
