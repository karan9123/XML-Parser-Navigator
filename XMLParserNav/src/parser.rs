use crate::errors::Error;
use crate::tokens::{AxesName, ValidToken};


pub static TOKEN_LIST: [(&'static str, ValidToken); 11] = [
    ("..", ValidToken::Parent),
    ("//", ValidToken::DoubleForwardSlash),
    (".", ValidToken::Period),
    ("/", ValidToken::ForwardSlash),
    ("[", ValidToken::LeftBracket),
    ("]", ValidToken::RightBracket),
    ("(", ValidToken::LeftParen),
    (")", ValidToken::RightParen),
    ("@", ValidToken::AtSign),
    ("::", ValidToken::LocationStep),
    ("=", ValidToken::Equal)
];


pub static AXES: [(&'static str, AxesName); 13] = [
    ("ancestor-or-self", AxesName::AncestorOrSelf),
    ("ancestor", AxesName::Ancestor),
    ("attribute", AxesName::Attribute),
    ("child", AxesName::Child),
    ("descendant-or-self", AxesName::DescendantOrSelf),
    ("descendant", AxesName::Descendant),
    ("following-sibling", AxesName::FollowingSibling),
    ("following", AxesName::Following),
    ("namespace", AxesName::Namespace),
    ("parent", AxesName::Parent),
    ("preceding-sibling", AxesName::PrecedingSibling),
    ("preceding", AxesName::Preceding),
    ("self", AxesName::SelfAxis),
];


pub struct Token {
    xpath: String,
    pos: usize,
}

impl Token {


    pub fn new(xpath: String) -> Self {
        Token {
            xpath,
            pos: 0,
        }
    }

    pub fn is_end(&self) -> bool {
        self.xpath.len() <= self.pos
    }


    pub fn next_func(&mut self) -> Result<ValidToken, Error> {
        /*let _remaining_path = {
            // let k = self.xpath.chars().nth(self.pos).unwrap();
            while self.xpath.chars().nth(self.pos).unwrap() == ' ' {
                self.pos += 1;
            }

            &self.xpath[self.pos..]
        };*/

        let found = self.parse_tokens(&TOKEN_LIST)
            .or_else(|| self.parse_axes(&AXES))
            .or_else(|| self.parse_literal())
            .or_else(|| self.parse_number());

        if let Some((token_size, token)) = found {
            self.pos += token_size;
            Ok(token)
        } else {
            self.pos = self.xpath.len();
            Err(Error::Token)
        }
    }

    fn parse_axes(&self, axes: &[(&'static str, AxesName)]) -> Option<(usize, ValidToken)> {
        for (id, axesname) in axes {
            let path_length = self.xpath.len().clone();
            let id_length = id.len().clone();
            if (self.pos + id_length) <= path_length {
                if self.xpath[self.pos..(self.pos + id.len())] == **id {
                    return Some((id.len(), ValidToken::Axes(axesname.clone())));
                }
            }
        }
        None
    }

    fn parse_number(&mut self) -> Option<(usize, ValidToken)> {
        let zero_ascii = '0' as u32;
        let mut end_pos: usize = 0;
        for i in self.xpath[self.pos..].chars() {
            if (zero_ascii <= i as u32) && (i as u32 <= zero_ascii + 10) {
                end_pos += 1;
            } else {
                break;
            }
        }
        if end_pos > 0 {
            return Some((end_pos, ValidToken::Number(self.xpath[self.pos..(self.pos + end_pos)].parse().unwrap())));
        }
        None
    }

    fn parse_literal(&mut self) -> Option<(usize, ValidToken)> {
        let a_ascii = 'a' as u32;
        let mut end_pos: usize = 0;
        for i in self.xpath[self.pos..].chars() {
            if (a_ascii <= i.to_ascii_lowercase() as u32) && (i.to_ascii_lowercase() as u32 <= a_ascii + 25) {
                end_pos += 1;
            } else {
                break;
            }
        }
        if end_pos > 0 {
            return Some((end_pos, ValidToken::Literal(self.xpath[self.pos..(self.pos + end_pos)].parse().unwrap())));
        }
        None
    }

    fn parse_tokens(&self, valid_tokens: &[(&'static str, ValidToken)]) -> Option<(usize, ValidToken)> {
        for (id, token) in valid_tokens {
            let path_length = self.xpath.len().clone();
            let id_length = id.len().clone();
            if (self.pos + id_length) <= path_length {
                if self.xpath[self.pos..(self.pos + id.len())] == **id {
                    // println!("printing from token");
                    return Some((id.len(), token.clone()));
                }
            }
        }
        None
    }
}

impl Iterator for Token {
    type Item = ValidToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            None
        } else {
            Some(self.next_func().unwrap())
        }
    }
}

