use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum CrossesValues {
    AutoZero,
    Maximum,
    Minimum,
}
impl Default for CrossesValues {
    fn default() -> Self {
        Self::AutoZero
    }
}
impl EnumTrait for CrossesValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::AutoZero => "autoZero",
            Self::Maximum => "max",
            Self::Minimum => "min",
        }
    }
}
impl FromStr for CrossesValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "autoZero" => Ok(Self::AutoZero),
            "max" => Ok(Self::Maximum),
            "min" => Ok(Self::Minimum),
            _ => Err(()),
        }
    }
}
