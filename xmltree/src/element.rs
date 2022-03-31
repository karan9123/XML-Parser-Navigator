use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use std::iter::Filter;
use std::slice::{Iter, IterMut};
use std::str::FromStr;
use std::string::ToString;

use crate::errors::XmlErrors;
use crate::tree::ElementTree;

/// An XML element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// Tag prefix, used for namespacing: `xsl` in `xsl:for-each`
    pub namespace: Option<String>,
    /// Tag name: `for-each` in `xsl:for-each`
    pub tag: String,
    /// Tag attributes
    pub attributes: HashMap<String, String>, // make &str from String
    /// A vector of child elements
    pub children: Vec<Element>,
    /// Contents of the element
    pub text: Option<String>,
    /// CDATA contents of the element
    pub cdata: Option<String>,
}


impl Default for Element {
    fn default() -> Self {
        Element {
            namespace: None,
            tag: "tag".to_owned(),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
            cdata: None,
        }
    }
}

impl Element {
    /// Create a new `Element` with the tag name `name`
    pub fn new<S>(name: S) -> Element
        where
            S: ToString,
    {
        Element {
            tag: name.to_string(),
            ..Element::default()
        }
    }

    /// Parse the contents of an element
    pub(crate) fn parse<R: Read>(
        &mut self,
        mut reader: &mut xml::reader::EventReader<R>,
    ) -> Result<(), XmlErrors> {
        use xml::reader::XmlEvent;

        loop {
            let ev = reader.next()?;
            match ev {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let mut attr_map = HashMap::new();
                    for attr in attributes {
                        let attr_name = match attr.name.prefix {
                            Some(prefix) => format!("{}:{}", prefix, attr.name.local_name),
                            None => attr.name.local_name,
                        };
                        attr_map.insert(attr_name, attr.value);
                    }

                    let mut child = Element {
                        namespace: name.prefix,
                        tag: name.local_name,
                        attributes: attr_map,
                        ..Element::default()
                    };
                    child.parse(&mut reader)?;
                    self.children.push(child);
                }
                XmlEvent::EndElement { name } => {
                    if name.prefix == self.namespace && name.local_name == self.tag {
                        return Ok(());
                    } else {
                        // This should never happen, since the base xml library will panic first
                        panic!("Unexpected closing tag: {}, expected {}", name, self.tag);
                    }
                }
                XmlEvent::Characters(s) => {
                    let text = match self.text {
                        Some(ref v) => v.clone(),
                        None => String::new(),
                    };
                    self.text = Some(text + &s);
                }
                XmlEvent::CData(s) => {
                    let cdata = match self.cdata {
                        Some(ref v) => v.clone(),
                        None => String::new(),
                    };
                    self.cdata = Some(cdata + &s);
                }
                XmlEvent::StartDocument { .. }
                | XmlEvent::EndDocument
                | XmlEvent::ProcessingInstruction { .. }
                | XmlEvent::Whitespace(_)
                | XmlEvent::Comment(_) => {}
            }
        }
    }

    /// Write an element and its contents to `writer`
    pub(crate) fn write<W: Write>(
        &self,
        writer: &mut xml::writer::EventWriter<W>,
    ) -> Result<(), XmlErrors> {
        use xml::attribute::Attribute;
        use xml::name::Name;
        use xml::namespace::Namespace;
        use xml::writer::XmlEvent;

        let name = Name::local(&self.tag);
        let mut attributes = Vec::with_capacity(self.attributes.len());
        for (k, v) in &self.attributes {
            attributes.push(Attribute {
                name: Name::local(k),
                value: v,
            });
        }

        let namespace = Namespace::empty();

        writer.write(XmlEvent::StartElement {
            name,
            attributes: Cow::Owned(attributes),
            namespace: Cow::Owned(namespace),
        }).unwrap();

        if let Some(ref text) = self.text {
            writer.write(XmlEvent::Characters(&text[..])).unwrap();
        }
        if let Some(ref cdata) = self.cdata {
            writer.write(XmlEvent::CData(&cdata[..])).unwrap();
        }

        for e in &self.children {
            e.write(writer)?;
        }

        writer.write(XmlEvent::EndElement { name: Some(name) }).unwrap();

        Ok(())
    }

    /// Find a single child of the current `Element`, given a predicate
    pub fn find_child<P>(&self, predicate: P) -> Option<&Element>
        where
            P: for<'r> Fn(&'r &Element) -> bool,
    {
        self.children.iter().find(predicate)
    }

    /// Find a single child of the current `Element`, given a predicate; returns a mutable borrow
    pub fn find_child_mut<P>(&mut self, predicate: P) -> Option<&mut Element>
        where
            P: for<'r> FnMut(&'r &mut Element) -> bool,
    {
        self.children.iter_mut().find(predicate)
    }

    /// Traverse element using an xpath-like string: root/child/a
    pub fn find(&self, path: &str) -> Result<&Element, XmlErrors> {
        Self::find_path(&path.split('/').collect::<Vec<&str>>(), path, self)
    }

    pub fn find_value<T: FromStr>(&self, path: &str) -> Result<Option<T>, XmlErrors> {
        let el = self.find(path)?;
        if let Some(text) = el.text.as_ref() {
            match T::from_str(text) {
                Err(_) => Err(XmlErrors::ValueFromStr {
                    t: text.to_string(),
                }),
                Ok(value) => Ok(Some(value)),
            }
        } else {
            Ok(None)
        }
    }

    fn find_path<'a>(
        path: &[&str],
        original: &str,
        tree: &'a Element,
    ) -> Result<&'a Element, XmlErrors> {
        if path.is_empty() {
            return Ok(tree);
        }

        match tree.find_child(|t| t.tag == path[0]) {
            Some(element) => Self::find_path(&path[1..], original, element),
            None => Err(XmlErrors::ElementNotFound { t: original.into() }),
        }
    }

    /// Filters the children of the current `Element`, given a predicate
    pub fn filter_children<P>(&self, predicate: P) -> Filter<Iter<Element>, P>
        where
            P: for<'r> Fn(&'r &Element) -> bool,
    {
        self.children.iter().filter(predicate)
    }

    /// Filters the children of the current `Element`, given a predicate; returns a mutable iterator
    pub fn filter_children_mut<P>(&mut self, predicate: P) -> Filter<IterMut<Element>, P>
        where
            P: for<'r> FnMut(&'r &mut Element) -> bool,
    {
        self.children.iter_mut().filter(predicate)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let doc = ElementTree {
            root: Some(self.clone()),
            ..ElementTree::default()
        };
        let mut v = Vec::<u8>::new();
        doc.write_with(&mut v, false, "  ", true).unwrap();
        let s = String::from_utf8(v).unwrap();
        f.write_str(&s[..])
    }
}





///`------------------------Builder starts from here-----------------------`

/// A builder for Element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementBuilder {
    /// The XML element we're working on
    element: Element,
}

impl ElementBuilder {
    /// Create a builder for an `Element` with the tag name `name`
    pub fn new<S>(name: S) -> ElementBuilder
        where
            S: ToString,
    {
        ElementBuilder {
            element: Element::new(name),
        }
    }

    /// Set the element's prefix to `prefix`
    pub fn prefix<S>(&mut self, prefix: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.namespace = Some(prefix.to_string());
        self
    }

    /// Set the element's attribute `key` to `value`
    pub fn attr<K, V>(&mut self, key: K, value: V) -> &mut ElementBuilder
        where
            K: ToString,
            V: ToString,
    {
        self.element
            .attributes
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Set the element's text to `text`
    pub fn text<S>(&mut self, text: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.text = Some(text.to_string());
        self
    }

    /// Set the element's CDATA to `cdata`
    pub fn cdata<S>(&mut self, cdata: S) -> &mut ElementBuilder
        where
            S: ToString,
    {
        self.element.cdata = Some(cdata.to_string());
        self
    }

    /// Append children to this `Element`
    pub fn children(&mut self, children: Vec<&mut ElementBuilder>) -> &mut ElementBuilder {
        self.element
            .children
            .append(&mut children.into_iter().map(|i| i.element()).collect());
        self
    }

    /// Creates an `Element` from the builder
    pub fn element(&self) -> Element {
        self.element.clone()
    }
}

impl From<ElementBuilder> for Element {
    fn from(value: ElementBuilder) -> Element {
        value.element()
    }
}

impl From<Element> for ElementBuilder {
    fn from(element: Element) -> ElementBuilder {
        ElementBuilder { element }
    }
}
