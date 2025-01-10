use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum DisplayBlanksAsValues {
    Gap,
    Span,
    Zero,
}
impl Default for DisplayBlanksAsValues {
    fn default() -> Self {
        Self::Span
    }
}
impl EnumTrait for DisplayBlanksAsValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Gap => "gap",
            Self::Span => "span",
            Self::Zero => "zero",
        }
    }
}
impl FromStr for DisplayBlanksAsValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "gap" => Ok(Self::Gap),
            "span" => Ok(Self::Span),
            "zero" => Ok(Self::Zero),
            _ => Err(()),
        }
    }
}
