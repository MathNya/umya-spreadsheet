use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CustomDocumentPropertyValue {
    String(Box<str>),
    Date(Box<str>),
    Numeric(i32),
    Bool(bool),
    Null,
}
impl fmt::Display for CustomDocumentPropertyValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(v) | Self::Date(v) => write!(f, "{v}"),
            Self::Numeric(v) => write!(f, "{}", &v),
            Self::Bool(v) => write!(f, "{}", if *v { "true" } else { "false" }),
            Self::Null => write!(f, ""),
        }
    }
}
impl Default for CustomDocumentPropertyValue {
    #[inline]
    fn default() -> Self {
        Self::Null
    }
}
impl CustomDocumentPropertyValue {
    #[inline]
    pub(crate) fn tag(&self) -> Option<&str> {
        match self {
            Self::String(_) => Some("vt:lpwstr"),
            Self::Date(_) => Some("vt:filetime"),
            Self::Numeric(_) => Some("vt:i4"),
            Self::Bool(_) => Some("vt:bool"),
            Self::Null => None,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use tag()")]
    pub(crate) fn get_tag(&self) -> Option<&str> {
        self.tag()
    }

    #[inline]
    pub(crate) fn number(&self) -> Option<i32> {
        match self {
            Self::Numeric(number) => Some(*number),
            _ => None,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use number()")]
    pub(crate) fn get_number(&self) -> Option<i32> {
        self.number()
    }

    #[inline]
    pub(crate) fn bool(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use bool()")]
    pub(crate) fn get_bool(&self) -> Option<bool> {
        self.bool()
    }
}
