use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use crate::errors::Error;
// use crate::{tree, tokens};
use crate::element::Element;
use crate::tokens::{AxesName, ValidToken};
use crate::parser::Token;
use crate::tree::ElementTree;


///This data structure takes in XML Tree and query
pub struct Bridge {
    pub tree: ElementTree,
    pub token: Token,
    pub token_steps: Vec<ValidToken>,
    pub error: Option<Error>,
}


impl Bridge {
    pub fn new(tree: ElementTree, xpath: String) -> Self {
        Bridge {
            tree,
            token: Token::new(xpath),
            token_steps: Vec::new(),
            error: None,
        }
    }


    ///fills up the token step vector from token string
    pub fn token_step_filler(&mut self) {
        while !self.token.is_end() {
            match self.token.next() {
                Some(token) => { self.expand_abbreviation(token) }
                None => {
                    self.error = Some(Error::Token);
                    return;
                }
            }
        }
    }


    ///changes tokens to final Axes version of themselves
    fn expand_abbreviation(&mut self, token: ValidToken) {
        match token {
            ValidToken::Period => {
                self.token_steps.extend([
                    ValidToken::Axes(AxesName::SelfAxis)
                ].iter().cloned());
            }
            ValidToken::AtSign => {
                self.token_steps.push(ValidToken::Axes(AxesName::Attribute));
            }
            ValidToken::Parent => {
                self.token_steps.extend([
                    ValidToken::Axes(AxesName::Parent)
                ].iter().cloned());
            }
            ValidToken::DoubleForwardSlash => {
                self.token_steps.extend([
                    ValidToken::Axes(AxesName::DescendantOrSelf)
                ].iter().cloned());
            }
            ValidToken::ForwardSlash => {
                self.token_steps.extend([
                    ValidToken::Axes(AxesName::Child)
                ].iter().cloned());
            }
            _ => {
                self.token_steps.push(token);
            }
        }
    }


    ///Method which makes sense of the tokenized XPath and the ElementTree passed to the Bridge
    pub fn produce(&mut self) -> Result<Vec<ReturnEnum>, Error>
    {
        let mut preceding_sibling_flag: bool = false;
        let mut follow_sibling_flag: bool = false;
        let mut equal_flag: bool = false;
        let mut attribute_name = String::new();
        let mut attribute_flag: bool = false;
        let mut namespace_flag: bool = false;
        let mut parent_flag: bool = false;
        let mut aos_flag: bool = false;
        let mut ancestor_flag: bool = false;
        let mut self_flag: bool = false;
        let mut is_valid_xpath: bool = false;
        let mut ret_vec: Vec<&Element> = Vec::new();
        let mut fin_ret_vec: Vec<ReturnEnum> = Vec::new();
        let mut child_flag: bool = false;
        let mut dos_flag: bool = false;
        let mut descendant_flag: bool = false;
        let root: &Element = self.tree.root.as_ref().unwrap();
        let mut bracket_open: bool = false;


        if self.token_steps.len() == 1 {
            return Err(Error::XPath);
        }

        if self.token_steps[0] != ValidToken::Axes(AxesName::DescendantOrSelf) &&
            self.token_steps[0] != ValidToken::Axes(AxesName::Child) {
            return Err(Error::XPath);
        }

        for token in self.token_steps.clone() {
            match token {
                ValidToken::Equal => {
                    if !attribute_flag || attribute_name.is_empty() {
                        return Err(Error::XPath);
                    } else {
                        attribute_flag = false;
                        equal_flag = true;
                    }
                }
                ValidToken::AtSign => {
                    if !bracket_open {
                        return Err(Error::XPath);
                    }
                }
                ValidToken::LeftBracket => {
                    bracket_open = true;
                }
                ValidToken::RightBracket => {
                    if bracket_open {
                        bracket_open = false;
                    } else {
                        return Err(Error::XPathOpenBracket);
                    }
                }
                // ValidToken::LeftParen => {}
                // ValidToken::RightParen => {}
                ValidToken::Literal(literal) => {
                    /*This is logic for literal after ancestor token*/if ancestor_flag {
                        let mut parent_map = HashMap::new();
                        let mut ancestor_vec = VecDeque::new();
                        ret_vec.clear();
                        // ret_vec.push(root);
                        ancestor_vec.push_back(root);

                        while let Some(element) = ancestor_vec.pop_front() {
                            for child in &element.children {
                                // if child.tag != literal{
                                if !parent_map.contains_key(child) {
                                    parent_map.insert(child, element);
                                    ancestor_vec.push_back(child);
                                }
                                // parent_map.entry(child).or_insert(element);
                                if child.tag == literal {
                                    ancestor_vec.clear(); //Comment this if you want all the paths instead of first
                                    let mut changing_child = child;
                                    while let Some(elem) = parent_map.get(changing_child) {
                                        ret_vec.push(*elem);
                                        changing_child = *elem;
                                    }
                                }
                            }
                        }
                        ancestor_flag = false;
                        if !aos_flag {
                            is_valid_xpath = true;
                        }
                    }
                    /*This is logic for literal after a child Token*/ if child_flag {
                        if root.tag == literal {
                            ret_vec.push(root)
                        } else if !ret_vec.is_empty() {
                            let mut old_element_vector = Vec::new();
                            while let Some(element) = ret_vec.pop() {
                                old_element_vector.push(element)
                            }

                            for old_element in old_element_vector {
                                for element in old_element.filter_children(|el| el.tag == literal) {
                                    ret_vec.push(element)
                                }
                            }
                        }
                        child_flag = false;
                        is_valid_xpath = true;
                    }
                    /*This is logic for literal after a descendant or self Token*/if dos_flag {
                        let mut nodes: VecDeque<&Element> = VecDeque::new();
                        if ret_vec.is_empty() {
                            nodes.push_back(root);
                        } else {
                            nodes.push_back(ret_vec.pop().unwrap());
                        }

                        if nodes[0].tag == literal {
                            ret_vec.push(nodes.pop_front().unwrap());
                            is_valid_xpath = true;
                            if !descendant_flag {
                                break;
                            }
                        }

                        while let Some(node) = nodes.pop_front() {
                            for child in &node.children {
                                if child.tag == literal {
                                    // ret_vec.clear();
                                    ret_vec.push(child);
                                    for child in &child.children {
                                        for grandchild in &child.children {
                                            ret_vec.push(grandchild)
                                        }
                                    }
                                } else {
                                    nodes.push_back(child)
                                }
                            }
                        }

                        dos_flag = false;
                        is_valid_xpath = true;
                    }
                    /* This removes self for descendant*/ if descendant_flag {
                        // ret_vec.pop();
                        descendant_flag = true;
                        is_valid_xpath = true;
                    }
                    /*This is logic for literal after self token*/if self_flag {
                        if ret_vec.is_empty() {
                            ret_vec.push(root);
                        } else {
                            let parent = ret_vec.pop().unwrap();
                            ret_vec.clear();
                            ret_vec.push(parent.find_child(|el| el.tag == literal).unwrap());
                            self_flag = false;
                            is_valid_xpath = true;
                        }
                    }
                    /*This is logic for literal after ancestor-or-self token*/if aos_flag {
                        let temp_vec = ret_vec.clone();
                        ret_vec.clear();
                        ret_vec.push(temp_vec[0].find_child(|el| el.tag == literal).unwrap());
                        for i in temp_vec {
                            ret_vec.push(i);
                        }
                        is_valid_xpath = true;
                    }
                    /*This is logic for literal after parent token*/if parent_flag {
                        let mut parent_map = HashMap::new();
                        let mut ancestor_vec = VecDeque::new();
                        ret_vec.clear();
                        ancestor_vec.push_back(root);

                        while let Some(element) = ancestor_vec.pop_front() {
                            for child in &element.children {
                                if !parent_map.contains_key(child) {
                                    parent_map.insert(child, element);
                                    ancestor_vec.push_back(child);
                                }
                                if child.tag == literal {
                                    ancestor_vec.clear();//Comment this if you want all the parents instead of first
                                    ret_vec.push(parent_map.get(child).unwrap());
                                }
                            }
                        }
                        ancestor_flag = false;
                        if !aos_flag {
                            is_valid_xpath = true;
                        }
                    }
                    /*This is logic for literal after attribute token*/if attribute_flag {
                        attribute_name = literal.clone();
                    }
                    /*This is logic for literal after equal token*/if equal_flag {
                        /*let mut final_el : &Element = &Element::default();
                        for element in &ret_vec {
                            if element.attributes.get(&*attribute_name).unwrap().to_string() == literal {
                                final_el = *element;
                            }
                        }
                        ret_vec.clear();
                        ret_vec.push(final_el);
                        is_valid_xpath = true;
                        break;*/

                        while let Some(element) = ret_vec.pop() {
                            if literal == element.attributes.get(&*attribute_name).unwrap().to_string() {
                                ret_vec.clear();
                                ret_vec.push(element);
                                is_valid_xpath = true;
                                break;
                            }
                        }
                    }
                    /*This is logic for literal after follow sibling token*/if follow_sibling_flag {
                        let mut found: bool = false;
                        let parent = ret_vec.pop().unwrap();
                        ret_vec.clear();
                        for sibling in &parent.children {
                            if found {
                                ret_vec.push(sibling);
                            }
                            if sibling.tag == literal {
                                found = true;
                            }
                        }
                        is_valid_xpath = true;
                        break;
                    }
                    /*This is logic for literal after preceding sibling token*/if preceding_sibling_flag {
                        let mut found: bool = false;
                        let parent = ret_vec.pop().unwrap();
                        ret_vec.clear();
                        for sibling in &parent.children {
                            if sibling.tag == literal {
                                found = true;
                            }
                            if !found {
                                ret_vec.push(sibling);
                            }
                        }
                        is_valid_xpath = true;
                        break;
                    }
                }
                ValidToken::Number(num) => {
                    if bracket_open {
                        if ret_vec.is_empty() {
                            return Err(Error::XPath);
                        } else {
                            if ret_vec.len() < num as usize {
                                return Err(Error::XPath);
                            }
                            let ret_el = ret_vec[num as usize - 1];
                            ret_vec.clear();
                            ret_vec.push(ret_el);
                        }
                    } else {
                        return Err(Error::XPath);
                    }
                }
                ValidToken::Axes(axes_name) => {
                    match axes_name {
                        AxesName::Ancestor => {
                            if !dos_flag {
                                return Err(Error::XPath);
                            }
                            dos_flag = false;
                            ancestor_flag = true;
                            is_valid_xpath = false;
                        }
                        AxesName::AncestorOrSelf => {
                            if !dos_flag {
                                return Err(Error::XPath);
                            }
                            dos_flag = false;
                            ancestor_flag = true;
                            is_valid_xpath = false;
                            aos_flag = true;
                        }
                        AxesName::Parent => {
                            parent_flag = true;
                        }
                        AxesName::Attribute => {
                            attribute_flag = true;
                            is_valid_xpath = false;
                        }
                        AxesName::Namespace => {
                            if child_flag {
                                child_flag = false;
                            }
                            namespace_flag = true;
                            is_valid_xpath = true;
                        }
                        AxesName::Child => {
                            child_flag = true;
                            is_valid_xpath = false;
                        }
                        AxesName::Descendant => {
                            descendant_flag = true;
                            dos_flag = true;
                        }
                        AxesName::DescendantOrSelf => {
                            dos_flag = true;
                            is_valid_xpath = false;
                        }
                        AxesName::SelfAxis => {
                            if child_flag {
                                child_flag = false;
                            }
                            is_valid_xpath = true;
                        }
                        AxesName::Following => {}//TBI
                        AxesName::FollowingSibling => {
                            parent_flag = true;
                            follow_sibling_flag = true;
                            is_valid_xpath = false;
                        }
                        AxesName::Preceding => {}//TBI
                        AxesName::PrecedingSibling => {
                            parent_flag = true;
                            preceding_sibling_flag = true;
                            is_valid_xpath = false;
                        }
                    }
                }
                _ => {}
            }
        }


        if bracket_open {
            is_valid_xpath = false;
        }

        if namespace_flag {
            let ret_val = ret_vec.pop().unwrap();
            return match ret_val.namespace.as_ref() {
                Some(t) => {
                    fin_ret_vec.push(ReturnEnum::ElementName(t.clone()));
                    Ok(fin_ret_vec)
                }
                None => {
                    Err(Error::NoNamespace)
                }
            };
        }
        return if is_valid_xpath {
            for i in ret_vec {
                fin_ret_vec.push(ReturnEnum::ElementNode(i))
            }
            Ok(fin_ret_vec)
        } else {
            Err(Error::XPath)
        };
    }
}

pub enum ReturnEnum<'a> {
    ElementNode(&'a Element),
    ElementName(String),
}

impl<'a> Display for ReturnEnum<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnEnum::ElementNode(element) => {
                write!(f, "{}", element)
            }
            ReturnEnum::ElementName(el_name) => {
                write!(f, "{}", el_name)
            }
        }
    }
}

impl<'a> Debug for ReturnEnum<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnEnum::ElementNode(element) => {
                write!(f, "{}", element)
            }
            ReturnEnum::ElementName(el_name) => {
                write!(f, "{}", el_name)
            }
        }
    }
}

impl<'a> Clone for ReturnEnum<'a> {
    fn clone(&self) -> Self {
        return match self {
            ReturnEnum::ElementNode(t) => {
                ReturnEnum::ElementNode(t)
            }
            ReturnEnum::ElementName(k) => {
                ReturnEnum::ElementName(k.clone())
            }
        };
    }
}