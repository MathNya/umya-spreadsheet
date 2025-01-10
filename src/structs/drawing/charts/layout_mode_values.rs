use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LayoutModeValues {
    Edge,
    Factor,
}
impl Default for LayoutModeValues {
    fn default() -> Self {
        Self::Edge
    }
}
impl EnumTrait for LayoutModeValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Edge => "edge",
            Self::Factor => "factor",
        }
    }
}
impl FromStr for LayoutModeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "edge" => Ok(Self::Edge),
            "factor" => Ok(Self::Factor),
            _ => Err(()),
        }
    }
}
