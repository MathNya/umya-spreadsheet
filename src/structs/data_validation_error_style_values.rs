use std::str::FromStr;

use super::EnumTrait;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DataValidationErrorStyleValues {
    Information,
    Stop,
    Warning,
}

impl EnumTrait for DataValidationErrorStyleValues {
    #[inline]
    fn value_string(&self) -> &str {
        match self {
            Self::Information => "information",
            Self::Stop => "stop",
            Self::Warning => "warning",
        }
    }
}

impl Default for DataValidationErrorStyleValues {
    #[inline]
    fn default() -> Self {
        Self::Stop
    }
}

impl FromStr for DataValidationErrorStyleValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "information" => Self::Information,
            "stop" => Self::Stop,
            "warning" => Self::Warning,
            _ => return Err(()),
        })
    }
}
