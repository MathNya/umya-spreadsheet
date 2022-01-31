use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum MarkerStyleValues {
    Auto,
    Circle,
    Dash,
    Diamond,
    Dot,
    None,
    Picture,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}
impl Default for MarkerStyleValues {
    fn default() -> Self {
        Self::Auto
    }
}
impl EnumTrait for MarkerStyleValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Auto => "auto",
            Self::Circle => "circle",
            Self::Dash => "dash",
            Self::Diamond => "diamond",
            Self::Dot => "dot",
            Self::None => "none",
            Self::Picture => "picture",
            Self::Plus => "plus",
            Self::Square => "square",
            Self::Star => "star",
            Self::Triangle => "triangle",
            Self::X => "x",
        }
    }
}
impl FromStr for MarkerStyleValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "auto" => Ok(Self::Auto),
            "circle" => Ok(Self::Circle),
            "dash" => Ok(Self::Dash),
            "diamond" => Ok(Self::Diamond),
            "dot" => Ok(Self::Dot),
            "none" => Ok(Self::None),
            "picture" => Ok(Self::Picture),
            "plus" => Ok(Self::Plus),
            "square" => Ok(Self::Square),
            "star" => Ok(Self::Star),
            "triangle" => Ok(Self::Triangle),
            "x" => Ok(Self::X),
            _ => Err(()),
        }
    }
}
