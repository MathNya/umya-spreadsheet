use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PaneStateValues {
    Frozen,
    FrozenSplit,
    Split,
}
impl Default for PaneStateValues {
    #[inline]
    fn default() -> Self {
        Self::Split
    }
}
impl EnumTrait for PaneStateValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Frozen => "frozen",
            Self::FrozenSplit => "frozenSplit",
            Self::Split => "split",
        }
    }
}
impl FromStr for PaneStateValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "frozen" => Ok(Self::Frozen),
            "frozenSplit" => Ok(Self::FrozenSplit),
            "split" => Ok(Self::Split),
            _ => Err(()),
        }
    }
}
