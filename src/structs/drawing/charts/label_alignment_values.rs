use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum LabelAlignmentValues {
    Center,
    Left,
    Right,
}
impl Default for LabelAlignmentValues {
    fn default() -> Self {
        Self::Center
    }
}
impl EnumTrait for LabelAlignmentValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Center => "ctr",
            Self::Left => "l",
            Self::Right => "r",
        }
    }
}
impl FromStr for LabelAlignmentValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ctr" => Ok(Self::Center),
            "l" => Ok(Self::Left),
            "r" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
