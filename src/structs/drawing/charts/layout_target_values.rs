use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum LayoutTargetValues {
    #[default]
    Inner,
    Outer,
}
impl EnumTrait for LayoutTargetValues {
    fn value_string(&self) -> &str {
        match &self {
            Self::Inner => "inner",
            Self::Outer => "outer",
        }
    }
}
impl FromStr for LayoutTargetValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "inner" => Ok(Self::Inner),
            "outer" => Ok(Self::Outer),
            _ => Err(()),
        }
    }
}
