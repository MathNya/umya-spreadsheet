use std::str::FromStr;

use super::EnumTrait;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum CellFormulaValues {
    Array,
    DataTable,
    Normal,
    Shared,
}
impl Default for CellFormulaValues {
    fn default() -> Self {
        Self::Normal
    }
}
impl EnumTrait for CellFormulaValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Array => "array",
            Self::DataTable => "dataTable",
            Self::Normal => "normal",
            Self::Shared => "shared",
        }
    }
}
impl FromStr for CellFormulaValues {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "array" => Ok(Self::Array),
            "dataTable" => Ok(Self::DataTable),
            "normal" => Ok(Self::Normal),
            "shared" => Ok(Self::Shared),
            _ => Err(()),
        }
    }
}
