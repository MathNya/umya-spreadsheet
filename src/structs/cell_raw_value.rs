use std::fmt;

use super::{
    RichText,
    Text,
};
use crate::CellErrorType;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub enum CellRawValue {
    String(Box<str>),
    RichText(RichText),
    Lazy(Box<str>),
    Numeric(f64),
    Bool(bool),
    Error(CellErrorType),
    #[default]
    Empty,
}
impl fmt::Display for CellRawValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::RichText(v) => write!(f, "{}", v.get_text()),
            Self::Numeric(v) => write!(f, "{}", &v),
            Self::Bool(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            Self::Error(e) => write!(f, "{e}"),
            _ => write!(f, ""),
        }
    }
}

impl CellRawValue {
    #[inline]
    #[must_use]
    pub fn data_type(&self) -> &str {
        match self {
            Self::String(_) | Self::RichText(_) => "s",
            Self::Numeric(_) => "n",
            Self::Bool(_) => "b",
            Self::Error(_) => "e",
            _ => "",
        }
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_type()")]
    pub fn get_data_type(&self) -> &str {
        self.data_type()
    }

    #[inline]
    pub(crate) fn text(&self) -> Option<Text> {
        match self {
            Self::String(_) | // _
            Self::Numeric(_) | // _
            Self::Bool(_) => {
                let mut text = Text::default();
                text.set_value(self.to_string());
                Some(text)
            }
            _ => None,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use text()")]
    pub(crate) fn get_text(&self) -> Option<Text> {
        self.text()
    }

    #[inline]
    pub(crate) fn number(&self) -> Option<f64> {
        match self {
            Self::Numeric(number) => Some(*number),
            _ => None,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use number()")]
    pub(crate) fn get_number(&self) -> Option<f64> {
        self.number()
    }

    #[inline]
    #[must_use]
    pub fn rich_text(&self) -> Option<RichText> {
        match self {
            Self::RichText(v) => Some(v.clone()),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rich_text()")]
    pub fn get_rich_text(&self) -> Option<RichText> {
        self.rich_text()
    }

    #[inline]
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(*self, CellRawValue::Error(_))
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(*self, CellRawValue::Empty)
    }
}
