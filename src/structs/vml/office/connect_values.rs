use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum ConnectValues {
    Custom,
    None,
    Rectangle,
    Segments,
}
impl Default for ConnectValues {
    fn default() -> Self {
        Self::None
    }
}
impl EnumTrait for ConnectValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Custom => "custom",
            Self::None => "none",
            Self::Rectangle => "rect",
            Self::Segments => "segments",
        }
    }
}
impl FromStr for ConnectValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "custom" => Ok(Self::Custom),
            "none" => Ok(Self::None),
            "rect" => Ok(Self::Rectangle),
            "segments" => Ok(Self::Segments),
            _ => Err(()),
        }
    }
}
