use std::str::FromStr;

use super::EnumTrait;
#[derive(Clone, Debug)]
pub enum PageOrderValues {
    DownThenOver,
    OverThenDown,
}
impl Default for PageOrderValues {
    #[inline]
    fn default() -> Self {
        Self::DownThenOver
    }
}
impl EnumTrait for PageOrderValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::DownThenOver => "downThenOver",
            Self::OverThenDown => "overThenDown",
        }
    }
}
impl FromStr for PageOrderValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "downThenOver" => Ok(Self::DownThenOver),
            "overThenDown" => Ok(Self::OverThenDown),
            _ => Err(()),
        }
    }
}
