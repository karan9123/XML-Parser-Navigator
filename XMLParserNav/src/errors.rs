use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum XmlErrors{
    ElementNotFound { t: String},
    ValueFromStr { t: String},
    ParseError{source: xml::reader::Error},
    WriteError{source: xml::writer::Error},
}
impl fmt::Display for XmlErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            XmlErrors::ElementNotFound {t}=> {write!(f, "element: {} not found!!", t)},
            XmlErrors::ValueFromStr {t}=>{write!(f, "value: {} can not be parsed", t)},
            XmlErrors::ParseError {source} =>{write!(f, "Parse Error for: {}", source)},
            XmlErrors::WriteError {source}=>{write!(f, "Write error for: {}", source)}
        }
    }
}

impl From<xml::reader::Error> for XmlErrors{
    fn from(err: xml::reader::Error) -> Self {
        XmlErrors::ParseError { source: err}
    }
}
impl From<xml::writer::Error> for XmlErrors{
    fn from(err: xml::writer::Error) -> Self {
        XmlErrors::WriteError { source: err }
    }
}

impl From<XmlErrors> for std::fmt::Error{
    fn from(_: XmlErrors) -> fmt::Error {
        fmt::Error{}
    }
}

#[derive(Debug)]
pub enum Error{
    Token,
    XPath,
    NoNamespace,
    XPathOpenBracket
}

impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self{
            Error::Token => {write!(f,"Invalid Token")}
            Error::XPath => {write!(f,"Incorrect path")}
            Error::NoNamespace => {write!(f,"No Namespace")}
            Error::XPathOpenBracket =>{write!(f,"No matching open bracket")}

        }
    }
}