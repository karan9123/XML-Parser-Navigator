use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxesName {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    Preceding,
    PrecedingSibling,
    SelfAxis,
}

impl Display for AxesName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AxesName::Ancestor => { write!(f, "Ancestor") }
            AxesName::AncestorOrSelf => { write!(f, "AncestorOrSelf") }
            AxesName::Attribute => { write!(f, "Attribute") }
            AxesName::Child => { write!(f, "Child") }
            AxesName::Descendant => { write!(f, "Descendant") }
            AxesName::DescendantOrSelf => { write!(f, "DescendantOrSelf") }
            AxesName::Following => { write!(f, "Following") }
            AxesName::FollowingSibling => { write!(f, "FollowingSibling") }
            AxesName::Namespace => { write!(f, "Namespace") }
            AxesName::Parent => { write!(f, "Parent") }
            AxesName::Preceding => { write!(f, "Preceding") }
            AxesName::PrecedingSibling => { write!(f, "PrecedingSibling") }
            AxesName::SelfAxis => { write!(f, "SelfAxis") }
        }
    }
}


#[derive(PartialEq)]
pub enum ValidToken {
    /// ..
    Parent,
    /// //
    DoubleForwardSlash,
    ///.
    Period,
    /// /
    ForwardSlash,
    ///[
    LeftBracket,
    ///]
    RightBracket,
    ///(
    LeftParen,
    /// )
    RightParen,
    /// =
    Equal,
    /// '@'
    AtSign,
    //This is attribute
    /// '::'
    LocationStep,
    Literal(String),
    Number(f64),
    Axes(AxesName),
}

impl Display for ValidToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidToken::Parent => { write!(f, "..") }
            ValidToken::DoubleForwardSlash => { write!(f, "//") }
            ValidToken::Period => { write!(f, ".") }
            ValidToken::ForwardSlash => { write!(f, "/") }
            ValidToken::LeftBracket => { write!(f, "[") }
            ValidToken::RightBracket => { write!(f, "]") }
            ValidToken::LeftParen => { write!(f, "(") }
            ValidToken::RightParen => { write!(f, ")") }
            ValidToken::Literal(t) => { write!(f, "{}", t) }
            ValidToken::Number(t) => { write!(f, "{}", t) }
            ValidToken::Axes(t) => { write!(f, "{}", t) }
            ValidToken::AtSign => { write!(f, "@") }
            ValidToken::LocationStep => { write!(f, "::") }
            ValidToken::Equal => { write!(f, "=") }
        }
    }
}

impl Clone for ValidToken {
    fn clone(&self) -> Self {
        match self {
            ValidToken::Parent => { ValidToken::Parent }
            ValidToken::DoubleForwardSlash => { ValidToken::DoubleForwardSlash }
            ValidToken::Period => { ValidToken::Period }
            ValidToken::ForwardSlash => { ValidToken::ForwardSlash }
            ValidToken::LeftBracket => { ValidToken::LeftBracket }
            ValidToken::RightBracket => { ValidToken::RightBracket }
            ValidToken::LeftParen => { ValidToken::LeftParen }
            ValidToken::RightParen => { ValidToken::RightParen }
            ValidToken::Literal(t) => { ValidToken::Literal(t.clone()) }
            ValidToken::Number(t) => { ValidToken::Number(t.clone()) }
            ValidToken::Axes(t) => { ValidToken::Axes(t.clone()) }
            ValidToken::AtSign => { ValidToken::AtSign }
            ValidToken::LocationStep => { ValidToken::LocationStep }
            ValidToken::Equal => { ValidToken::Equal }
        }
    }
}
