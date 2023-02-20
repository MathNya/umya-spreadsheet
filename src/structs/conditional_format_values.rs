use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConditionalFormatValues {
    AboveAverage,
    BeginsWith,
    CellIs,
    ColorScale,
    ContainsBlanks,
    ContainsErrors,
    ContainsText,
    DataBar,
    DuplicateValues,
    EndsWith,
    Expression,
    IconSet,
    NotContainsBlanks,
    NotContainsErrors,
    NotContainsText,
    TimePeriod,
    Top10,
    UniqueValues,
}
impl Default for ConditionalFormatValues {
    fn default() -> Self {
        Self::Expression
    }
}
impl EnumTrait for ConditionalFormatValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::AboveAverage => "aboveAverage",
            Self::BeginsWith => "beginsWith",
            Self::CellIs => "cellIs",
            Self::ColorScale => "colorScale",
            Self::ContainsBlanks => "containsBlanks",
            Self::ContainsErrors => "containsErrors",
            Self::ContainsText => "containsText",
            Self::DataBar => "dataBar",
            Self::DuplicateValues => "duplicateValues",
            Self::EndsWith => "endsWith",
            Self::Expression => "expression",
            Self::IconSet => "iconSet",
            Self::NotContainsBlanks => "notContainsBlanks",
            Self::NotContainsErrors => "notContainsErrors",
            Self::NotContainsText => "notContainsText",
            Self::TimePeriod => "timePeriod",
            Self::Top10 => "top10",
            Self::UniqueValues => "uniqueValues",
        }
    }
}
impl FromStr for ConditionalFormatValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "aboveAverage" => Ok(Self::AboveAverage),
            "beginsWith" => Ok(Self::BeginsWith),
            "cellIs" => Ok(Self::CellIs),
            "colorScale" => Ok(Self::ColorScale),
            "containsBlanks" => Ok(Self::ContainsBlanks),
            "containsErrors" => Ok(Self::ContainsErrors),
            "containsText" => Ok(Self::ContainsText),
            "dataBar" => Ok(Self::DataBar),
            "duplicateValues" => Ok(Self::DuplicateValues),
            "endsWith" => Ok(Self::EndsWith),
            "expression" => Ok(Self::Expression),
            "iconSet" => Ok(Self::IconSet),
            "notContainsBlanks" => Ok(Self::NotContainsBlanks),
            "notContainsErrors" => Ok(Self::NotContainsErrors),
            "notContainsText" => Ok(Self::NotContainsText),
            "timePeriod" => Ok(Self::TimePeriod),
            "top10" => Ok(Self::Top10),
            "uniqueValues" => Ok(Self::UniqueValues),
            _ => Err(()),
        }
    }
}
