use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum LabelAlignmentValues {
    #[default]
    Center,
    Left,
    Right,
}
impl EnumTrait for LabelAlignmentValues {
    fn value_string(&self) -> &str {
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
