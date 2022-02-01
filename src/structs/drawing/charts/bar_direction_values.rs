use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum BarDirectionValues {
    Bar,
    Column,
}
impl Default for BarDirectionValues {
    fn default() -> Self {
        Self::Bar
    }
}
impl EnumTrait for BarDirectionValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Bar => "bar",
            Self::Column => "col",
        }
    }
}
impl FromStr for BarDirectionValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bar" => Ok(Self::Bar),
            "col" => Ok(Self::Column),
            _ => Err(()),
        }
    }
}
