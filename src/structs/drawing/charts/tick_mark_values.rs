use super::super::super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
#[derive(Default)]
pub enum TickMarkValues {
    #[default]
    Cross,
    Inside,
    None,
    Outside,
}

impl EnumTrait for TickMarkValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Cross => "cross",
            Self::Inside => "in",
            Self::None => "none",
            Self::Outside => "out",
        }
    }
}
impl FromStr for TickMarkValues {
    type Err = ();
    fn from_str(input: &str) -> Result<TickMarkValues, Self::Err> {
        match input {
            "cross" => Ok(Self::Cross),
            "in" => Ok(Self::Inside),
            "none" => Ok(Self::None),
            "out" => Ok(Self::Outside),
            _ => Err(()),
        }
    }
}
