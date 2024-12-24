use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PenAlignmentValues {
    Center,
    Insert,
}
impl Default for PenAlignmentValues {
    #[inline]
    fn default() -> Self {
        Self::Center
    }
}
impl EnumTrait for PenAlignmentValues {
    #[inline]
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Center => "ctr",
            Self::Insert => "in",
        }
    }
}
impl FromStr for PenAlignmentValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ctr" => Ok(Self::Center),
            "in" => Ok(Self::Insert),
            _ => Err(()),
        }
    }
}
