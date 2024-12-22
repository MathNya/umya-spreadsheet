use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum VerticalAlignmentRunValues {
    Baseline,
    Subscript,
    Superscript,
}
impl Default for VerticalAlignmentRunValues {
    #[inline]
    fn default() -> Self {
        Self::Baseline
    }
}
impl EnumTrait for VerticalAlignmentRunValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Baseline => "baseline",
            Self::Subscript => "subscript",
            Self::Superscript => "superscript",
        }
    }
}
impl FromStr for VerticalAlignmentRunValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "baseline" => Ok(Self::Baseline),
            "subscript" => Ok(Self::Subscript),
            "superscript" => Ok(Self::Superscript),
            _ => Err(()),
        }
    }
}
