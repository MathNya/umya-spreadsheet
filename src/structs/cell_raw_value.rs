use super::RichText;
use super::Text;
use std::fmt;

/// An enum to represent all different data types that can appear as
/// a value in a worksheet cell
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub enum CellRawValue {
    String(String),
    Str(String),
    RichText(RichText),
    Lazy(String),
    Numeric(f64),
    Bool(bool),
    Inline,
    Error,
    #[default]
    Empty,
}
impl fmt::Display for CellRawValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::RichText(v) => write!(f, "{}", v.get_text()),
            Self::Numeric(v) => write!(f, "{}", &v),
            Self::Bool(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            _ => write!(f, ""),
        }
    }
}

impl CellRawValue {
    pub fn get_data_type(&self) -> &str {
        match self {
            Self::String(_) => "s",
            Self::Str(_) => "str",
            Self::RichText(_) => "s",
            Self::Numeric(_) => "n",
            Self::Bool(_) => "b",
            Self::Error => "e",
            _ => "",
        }
    }

    pub(crate) fn get_text(&self) -> Option<Text> {
        match self {
            Self::String(_) | // _
            Self::Str(_) | // _
            Self::Numeric(_) | // _
            Self::Bool(_) => {
                let mut text = Text::default();
                text.set_value(self.to_string());
                Some(text)
            }
            _ => None,
        }
    }

    pub(crate) fn get_number(&self) -> Option<f64> {
        match self {
            Self::Numeric(number) => Some(*number),
            _ => None,
        }
    }

    pub fn get_rich_text(&self) -> Option<RichText> {
        match self {
            Self::RichText(v) => Some(v.clone()),
            _ => None,
        }
    }
}
