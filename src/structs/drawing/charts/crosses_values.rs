use std::str::FromStr;

use super::super::super::EnumTrait;
#[derive(Clone, Debug, Default)]
pub enum CrossesValues {
    #[default]
    AutoZero,
    Maximum,
    Minimum,
}
impl EnumTrait for CrossesValues {
    fn value_string(&self) -> &str {
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
