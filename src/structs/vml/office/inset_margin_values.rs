use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum InsetMarginValues {
    #[default]
    Auto,
    Custom,
}
impl EnumTrait for InsetMarginValues {
    fn value_string(&self) -> &str {
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
