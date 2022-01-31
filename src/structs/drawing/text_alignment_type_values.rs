use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum TextAlignmentTypeValues {
    Center,
    Distributed,
    Justified,
    JustifiedLow,
    Left,
    Right,
    ThaiDistributed,
}
impl Default for TextAlignmentTypeValues {
    fn default() -> Self {
        Self::Left
    }
}
impl EnumTrait for TextAlignmentTypeValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Center => "ctr",
            Self::Distributed => "dist",
            Self::Justified => "just",
            Self::JustifiedLow => "justLow",
            Self::Left => "l",
            Self::Right => "r",
            Self::ThaiDistributed => "thaiDist",
        }
    }
}
impl FromStr for TextAlignmentTypeValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ctr" => Ok(Self::Center),
            "dist" => Ok(Self::Distributed),
            "just" => Ok(Self::Justified),
            "justLow" => Ok(Self::JustifiedLow),
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            "thaiDist" => Ok(Self::ThaiDistributed),
            _ => Err(()),
        }
    }
}
