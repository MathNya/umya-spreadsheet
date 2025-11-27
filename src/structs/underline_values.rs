use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UnderlineValues {
    Double,
    DoubleAccounting,
    None,
    Single,
    SingleAccounting,
}
impl Default for UnderlineValues {
    #[inline]
    fn default() -> Self {
        Self::Single
    }
}
impl EnumTrait for UnderlineValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Double => "double",
            Self::DoubleAccounting => "doubleAccounting",
            Self::None => "none",
            Self::Single => "single",
            Self::SingleAccounting => "singleAccounting",
        }
    }
}
impl FromStr for UnderlineValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "double" => Ok(Self::Double),
            "doubleAccounting" => Ok(Self::DoubleAccounting),
            "none" => Ok(Self::None),
            "single" => Ok(Self::Single),
            "singleAccounting" => Ok(Self::SingleAccounting),
            _ => Err(()),
        }
    }
}
