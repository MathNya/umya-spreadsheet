use super::EnumTrait;
use std::str::FromStr;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum SourceValues {
    Consolidation,
    External,
    Scenario,
    Worksheet,
}
impl Default for SourceValues {
    fn default() -> Self {
        Self::Worksheet
    }
}
impl EnumTrait for SourceValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Consolidation => "consolidation",
            Self::External => "external",
            Self::Scenario => "scenario",
            Self::Worksheet => "worksheet",
        }
    }
}
impl FromStr for SourceValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "consolidation" => Ok(Self::Consolidation),
            "external" => Ok(Self::External),
            "scenario" => Ok(Self::Scenario),
            "worksheet" => Ok(Self::Worksheet),
            _ => Err(()),
        }
    }
}
