use crate::from_err;
use std::{fmt, str::FromStr};

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
            _ => Err(XlsxError::CellError(s.into())),
        }
    }
}

#[derive(Debug)]
pub enum XlsxError {
    /// IO error
    Io(std::io::Error),
    /// Xml error
    Xml(quick_xml::Error),
    /// Zip error
    Zip(zip::result::ZipError),

    Uft8(std::string::FromUtf8Error),
    /// Cell error
    CellError(String),
}

from_err!(std::io::Error, XlsxError, Io);
from_err!(quick_xml::Error, XlsxError, Xml);
from_err!(zip::result::ZipError, XlsxError, Zip);
from_err!(std::string::FromUtf8Error, XlsxError, Uft8);

impl fmt::Display for XlsxError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::XlsxError::*;
        match self {
            Io(i) => write!(f, "IoError: {}", i),
            Xml(s) => write!(f, "XmlError: {}", s),
            Zip(s) => write!(f, "ZipError: {}", s),
            Uft8(s) => write!(f, "Uft8Error: {}", s),
            CellError(e) => write!(f, "Unsupported cell error value '{e}'"),
        }
    }
}
impl std::error::Error for XlsxError {}
