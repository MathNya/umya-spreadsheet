use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub enum SheetViewValues {
    Normal,
    PageBreakPreview,
    PageLayout,
}
impl Default for SheetViewValues {
    fn default() -> Self {
        Self::Normal
    }
}
impl EnumTrait for SheetViewValues {
    fn get_value_string(&self) -> &str {
        match &self {
            Self::Normal => "normal",
            Self::PageBreakPreview => "pageBreakPreview",
            Self::PageLayout => "pageLayout",
        }
    }
}
impl FromStr for SheetViewValues {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "normal" => Ok(Self::Normal),
            "pageBreakPreview" => Ok(Self::PageBreakPreview),
            "pageLayout" => Ok(Self::PageLayout),
            _ => Err(()),
        }
    }
}
