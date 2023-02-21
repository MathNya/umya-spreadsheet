use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum PenAlignmentValues {
    Center,
    Insert,
}
impl Default for PenAlignmentValues {
    fn default() -> Self {
        Self::Center
    }
}
impl EnumTrait for PenAlignmentValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Center => "ctr",
            Self::Insert => "in",
        }
    }
}
impl FromStr for PenAlignmentValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ctr" => Ok(Self::Center),
            "in" => Ok(Self::Insert),
            _ => Err(()),
        }
    }
}
