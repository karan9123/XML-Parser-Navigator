use std::borrow::Cow;
use std::fmt;
use std::fmt::write;
use std::io::Error;
use xml::common::Position;
use xml::reader::ErrorKind;

#[derive(Debug)]
pub enum XmlErrors{
    ElementNotFound { t: String},
    ValueFromStr { t: String},
    ParseError{source: xml::reader::Error},
    WriteError{source: xml::writer::Error},
    //ParseError and WriteError: to see if needed
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
/*impl<'a, P, M> From<(&'a P, M)> for XmlErrors where P: Position, M: Into<Cow<'static, str>> {
    fn from(orig: (&'a P, M)) -> Self {
        let k:xml::reader::Error  = xml::reader::Error{
            pos: orig.0.position(),
            kind: (&event_reader, "custom error").into()
        };
        XmlErrors::ParseError { source: k}
    }
}*/