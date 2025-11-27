use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum DisplayBlanksAsValues {
    Gap,
    #[default]
    Span,
    Zero,
}
impl EnumTrait for DisplayBlanksAsValues {
    fn value_string(&self) -> &str {
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
