use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum InsetMarginValues {
    Auto,
    Custom,
}
impl Default for InsetMarginValues {
    fn default() -> Self {
        Self::Auto
    }
}
impl EnumTrait for InsetMarginValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Auto => "auto",
            Self::Custom => "custom",
        }
    }
}
impl FromStr for InsetMarginValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "auto" => Ok(Self::Auto),
            "custom" => Ok(Self::Custom),
            _ => Err(()),
        }
    }
}
