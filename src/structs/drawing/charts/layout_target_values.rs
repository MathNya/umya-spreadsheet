use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LayoutTargetValues {
    Inner,
    Outer,
}
impl Default for LayoutTargetValues {
    fn default() -> Self {
        Self::Inner
    }
}
impl EnumTrait for LayoutTargetValues {
    fn get_value_string(&self) -> &str {
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
