use std::str::FromStr;

use super::EnumTrait;
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum PivotTableAxisValues {
    AxisColumn,
    AxisPage,
    AxisRow,
    AxisValues,
}
impl Default for PivotTableAxisValues {
    #[inline]
    fn default() -> Self {
        Self::AxisValues
    }
}
impl EnumTrait for PivotTableAxisValues {
    #[inline]
    fn value_string(&self) -> &str {
        match &self {
            Self::AxisColumn => "axisCol",
            Self::AxisPage => "axisPage",
            Self::AxisRow => "axisRow",
            Self::AxisValues => "axisValues",
        }
    }
}
impl FromStr for PivotTableAxisValues {
    type Err = ();

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "axisCol" => Ok(Self::AxisColumn),
            "axisPage" => Ok(Self::AxisPage),
            "axisRow" => Ok(Self::AxisRow),
            "axisValues" => Ok(Self::AxisValues),
            _ => Err(()),
        }
    }
}
