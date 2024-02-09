use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CustomDocumentPropertyValue {
    String(String),
    Date(String),
    Numeric(i32),
    Bool(bool),
    Null,
}
impl fmt::Display for CustomDocumentPropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Date(v) => write!(f, "{}", v),
            Self::Numeric(v) => write!(f, "{}", &v),
            Self::Bool(v) => write!(f, "{}", if *v { "true" } else { "false" }),
            _ => write!(f, ""),
        }
    }
}
impl Default for CustomDocumentPropertyValue {
    fn default() -> Self {
        Self::Null
    }
}
impl CustomDocumentPropertyValue {
    pub(crate) fn get_tag(&self) -> Option<&str> {
        match self {
            Self::String(_) => Some("vt:lpwstr"),
            Self::Date(_) => Some("vt:filetime"),
            Self::Numeric(_) => Some("vt:i4"),
            Self::Bool(_) => Some("vt:bool"),
            _ => None,
        }
    }

    pub(crate) fn get_number(&self) -> Option<i32> {
        match self {
            Self::Numeric(number) => Some(*number),
            _ => None,
        }
    }

    pub(crate) fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }
}
