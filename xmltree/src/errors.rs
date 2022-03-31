use std::fmt;

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