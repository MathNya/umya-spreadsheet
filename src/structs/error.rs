use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt,
    str::FromStr,
};

// https://msdn.microsoft.com/en-us/library/office/ff839168.aspx
/// An enum to represent all different errors that can appear as
/// a value in a worksheet cell
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CellErrorType {
    /// Division by 0 error
    Div0,
    /// Invalid name error
    Name,
    /// Unavailable value error
    NA,
    /// Number error
    Num,
    /// Value error
    Value,
    /// Invalid cell reference error
    Ref,
    /// Null value error
    Null,
    /// Getting data
    Data,
}

impl fmt::Display for CellErrorType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            CellErrorType::Div0 => write!(f, "#DIV/0!"),
            CellErrorType::NA => write!(f, "#N/A"),
            CellErrorType::Name => write!(f, "#NAME?"),
            CellErrorType::Null => write!(f, "#NULL!"),
            CellErrorType::Num => write!(f, "#NUM!"),
            CellErrorType::Ref => write!(f, "#REF!"),
            CellErrorType::Value => write!(f, "#VALUE!"),
            CellErrorType::Data => write!(f, "#DATA!"),
        }
    }
}
impl FromStr for CellErrorType {
    type Err = XlsxError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#DIV/0!" => Ok(CellErrorType::Div0),
            "#N/A" => Ok(CellErrorType::NA),
            "#NAME?" => Ok(CellErrorType::Name),
            "#NULL!" => Ok(CellErrorType::Null),
            "#NUM!" => Ok(CellErrorType::Num),
            "#REF!" => Ok(CellErrorType::Ref),
            "#VALUE!" => Ok(CellErrorType::Value),
            "#DATA!" => Ok(CellErrorType::Data),
            _ => Err(XlsxError::with_message(
                XlsxErrorKind::InvalidCellErrorValue,
                format!("unsupported cell error value '{s}'"),
            )),
        }
    }
}

/// The category of an [`XlsxError`].
///
/// This enum is `#[non_exhaustive]`: new kinds may be added in minor releases,
/// so a `match` on it must include a wildcard arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum XlsxErrorKind {
    /// An I/O operation failed.
    Io,
    /// The XML content could not be read or written.
    Xml,
    /// The ZIP container could not be read or written.
    Zip,
    /// A byte sequence was not valid UTF-8.
    Utf8,
    /// A cell error value (e.g. `#DIV/0!`) is not supported.
    InvalidCellErrorValue,
    /// The requested item (such as a worksheet) was not found.
    NotFound,
    /// The worksheet has not been deserialized yet.
    /// Call [`Workbook::read_sheet`](crate::Workbook::read_sheet) first.
    NotDeserialized,
    /// The name is already used.
    NameDuplicate,
    /// An argument is invalid.
    InvalidArgument,
    /// An encryption or decryption operation failed.
    Crypto,
}

impl XlsxErrorKind {
    fn description(self) -> &'static str {
        match self {
            Self::Io => "I/O error",
            Self::Xml => "XML error",
            Self::Zip => "ZIP error",
            Self::Utf8 => "UTF-8 error",
            Self::InvalidCellErrorValue => "invalid cell error value",
            Self::NotFound => "not found",
            Self::NotDeserialized => {
                "worksheet is not deserialized; call Workbook::read_sheet first"
            }
            Self::NameDuplicate => "name is duplicated",
            Self::InvalidArgument => "invalid argument",
            Self::Crypto => "cryptographic operation failed",
        }
    }
}

impl fmt::Display for XlsxErrorKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// The error type of this crate.
///
/// The details are kept private so that more context can be added later
/// without breaking changes. Use [`XlsxError::kind`] to inspect the category
/// and [`std::error::Error::source`] to get the underlying error.
///
/// # Examples
/// ```
/// use umya_spreadsheet::XlsxErrorKind;
///
/// let book = umya_spreadsheet::new_file();
/// let err = book.sheet(10).unwrap_err();
/// assert_eq!(err.kind(), XlsxErrorKind::NotFound);
/// ```
pub struct XlsxError {
    inner: Box<Inner>,
}

struct Inner {
    kind:    XlsxErrorKind,
    message: Option<Cow<'static, str>>,
    source:  Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl XlsxError {
    /// Creates an error of the given kind.
    #[inline]
    #[must_use]
    pub fn new(kind: XlsxErrorKind) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: None,
            }),
        }
    }

    /// Creates an error of the given kind with an additional message.
    #[inline]
    #[must_use]
    pub fn with_message<M: Into<Cow<'static, str>>>(kind: XlsxErrorKind, message: M) -> Self {
        let mut err = Self::new(kind);
        err.inner.message = Some(message.into());
        err
    }

    /// Creates an error of the given kind caused by `source`.
    #[inline]
    #[must_use]
    pub fn with_source<E>(kind: XlsxErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        let mut err = Self::new(kind);
        err.inner.source = Some(source.into());
        err
    }

    /// Returns the category of this error.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> XlsxErrorKind {
        self.inner.kind
    }

    /// Returns the additional message, if any.
    #[inline]
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }
}

impl From<XlsxErrorKind> for XlsxError {
    #[inline]
    fn from(kind: XlsxErrorKind) -> Self {
        Self::new(kind)
    }
}

macro_rules! from_source {
    ($from:ty, $kind:ident) => {
        impl From<$from> for XlsxError {
            #[inline]
            fn from(e: $from) -> Self {
                Self::with_source(XlsxErrorKind::$kind, e)
            }
        }
    };
}

from_source!(std::io::Error, Io);
from_source!(quick_xml::Error, Xml);
from_source!(zip::result::ZipError, Zip);
from_source!(std::string::FromUtf8Error, Utf8);
from_source!(quick_xml::escape::EscapeError, Xml);
from_source!(quick_xml::events::attributes::AttrError, Xml);
from_source!(quick_xml::encoding::EncodingError, Xml);

impl fmt::Debug for XlsxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("XlsxError");
        d.field("kind", &self.inner.kind);
        if let Some(message) = &self.inner.message {
            d.field("message", message);
        }
        if let Some(source) = &self.inner.source {
            d.field("source", source);
        }
        d.finish()
    }
}

impl fmt::Display for XlsxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.kind)?;
        if let Some(message) = &self.inner.message {
            write!(f, ": {message}")?;
        }
        if let Some(source) = &self.inner.source {
            write!(f, ": {source}")?;
        }
        Ok(())
    }
}

impl StdError for XlsxError {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.inner.source {
            Some(source) => Some(&**source),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}
        assert_send_sync::<XlsxError>();
    }

    #[test]
    fn kind_and_source() {
        let err = XlsxError::from(std::io::Error::other("boom"));
        assert_eq!(err.kind(), XlsxErrorKind::Io);
        assert_eq!(err.to_string(), "I/O error: boom");
        assert!(err.source().is_some());

        let err = XlsxError::with_message(XlsxErrorKind::NotFound, "worksheet 'A'");
        assert_eq!(err.kind(), XlsxErrorKind::NotFound);
        assert_eq!(err.message(), Some("worksheet 'A'"));
        assert_eq!(err.to_string(), "not found: worksheet 'A'");
        assert!(err.source().is_none());
    }

    #[test]
    fn cell_error_type_from_str() {
        let err = "#FOO!".parse::<CellErrorType>().unwrap_err();
        assert_eq!(err.kind(), XlsxErrorKind::InvalidCellErrorValue);
    }
}
