use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PrintErrorValues {
    Blank,
    Dash,
    Displayed,
    NA,
}
impl Default for PrintErrorValues {
    #[inline]
    fn default() -> Self {
        Self::Displayed
    }
}
impl EnumTrait for PrintErrorValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::Blank => "blank",
            Self::Dash => "dash",
            Self::Displayed => "displayed",
            Self::NA => "NA",
        }
    }
}
impl FromStr for PrintErrorValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "blank" => Ok(Self::Blank),
            "dash" => Ok(Self::Dash),
            "displayed" => Ok(Self::Displayed),
            "NA" => Ok(Self::NA),
            _ => Err(()),
        }
    }
}
