use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceModeValues {
    A1,
    R1C1,
}
impl Default for ReferenceModeValues {
    #[inline]
    fn default() -> Self {
        Self::A1
    }
}
impl EnumTrait for ReferenceModeValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::A1 => "A1",
            Self::R1C1 => "R1C1",
        }
    }
}
impl FromStr for ReferenceModeValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A1" => Ok(Self::A1),
            "R1C1" => Ok(Self::R1C1),
            _ => Err(()),
        }
    }
}
