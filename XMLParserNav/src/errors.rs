use std::fmt;
use std::fmt::{Display, Formatter};

/// Custom error types for XML-related errors
#[derive(Debug)]
pub enum XmlErrors {
    /// Error indicating that an element was not found
    ElementNotFound { t: String },
    /// Error indicating that a value cannot be parsed
    ValueFromStr { t: String },
    /// Error indicating a parse error
    ParseError { source: xml::reader::Error },
    /// Error indicating a write error
    WriteError { source: xml::writer::Error },
}

impl fmt::Display for XmlErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlErrors::ElementNotFound { t } => write!(f, "Element '{}' not found", t),
            XmlErrors::ValueFromStr { t } => write!(f, "Value '{}' cannot be parsed", t),
            XmlErrors::ParseError { source } => write!(f, "Parse Error: {}", source),
            XmlErrors::WriteError { source } => write!(f, "Write Error: {}", source),
        }
    }
}

impl From<xml::reader::Error> for XmlErrors {
    fn from(err: xml::reader::Error) -> Self {
        XmlErrors::ParseError { source: err }
    }
}

impl From<xml::writer::Error> for XmlErrors {
    fn from(err: xml::writer::Error) -> Self {
        XmlErrors::WriteError { source: err }
    }
}

impl From<XmlErrors> for std::fmt::Error {
    fn from(_: XmlErrors) -> fmt::Error {
        fmt::Error {}
    }
}

/// Custom error types for XML-related errors
#[derive(Debug)]
pub enum Error {
    /// Error indicating an invalid token
    Token,
    /// Error indicating an incorrect path
    XPath,
    /// Error indicating no namespace
    NoNamespace,
    /// Error indicating no matching open bracket
    XPathOpenBracket,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Token => write!(f, "Invalid Token"),
            Error::XPath => write!(f, "Incorrect path"),
            Error::NoNamespace => write!(f, "No Namespace"),
            Error::XPathOpenBracket => write!(f, "No matching open bracket"),
        }
    }
}
