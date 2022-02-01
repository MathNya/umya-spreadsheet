use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum OfPieValues {
    Bar,
    Pie,
}
impl Default for OfPieValues {
    fn default() -> Self {
        Self::Pie
    }
}
impl EnumTrait for OfPieValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bar => "bar",
            Self::Pie => "pie",
        }
    }
}
impl FromStr for OfPieValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bar" => Ok(Self::Bar),
            "pie" => Ok(Self::Pie),
            _ => Err(()),
        }
    }
}
