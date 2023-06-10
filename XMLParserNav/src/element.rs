use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::iter::Filter;
use std::slice::Iter;
use crate::errors::XmlErrors;
use crate::tree::ElementTree;

/// An XML element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// The namespace of the element
    pub namespace: Option<String>,
    /// The tag name of the element
    pub tag: String,
    /// The attributes of the element
    pub attributes: HashMap<String, String>,
    /// The child elements of the element
    pub children: Vec<Element>,
    /// The text content of the element
    pub text: Option<String>,
}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl Default for Element {
    // for use in creating child element.
    fn default() -> Self {
        Element {
            namespace: None,
            tag: "tag".to_owned(),
            attributes: HashMap::new(),
            children: vec![],
            text: None,
        }
    }
}

impl Element {

 /*   ///Creates a new element by taking in, it's name
    pub fn new<S>(name: S) -> Element
        where
            S: ToString,
    {
        Element {
            tag: name.to_string(),
            // call implemented default within new.
            ..Element::default()
        }
    }*/

    /// Parse the contents of an element
    pub(crate) fn parse<R: Read>(
        &mut self,
        mut xml_reader: &mut xml::reader::EventReader<R>,
    ) -> Result<(), XmlErrors> {
        use xml::reader::XmlEvent;

        loop {
            let xml_event_type = xml_reader.next()?;
            match xml_event_type {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    ..
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
                    child.parse(&mut xml_reader)?;
                    self.children.push(child);
                }
                XmlEvent::EndElement { name } => {
                    if name.prefix == self.namespace && name.local_name == self.tag {
                        return Ok(());
                    } else {
                        panic!("Unexpected closing tag: {}, expected {}", name, self.tag);
                    }
                }
                XmlEvent::Characters(s) => {
                    let text = match &self.text {
                        Some(v) => v.clone(),
                        None => String::new(),
                    };
                    self.text = Some(text + &s);
                }
                XmlEvent::StartDocument { .. }
                | XmlEvent::EndDocument
                | XmlEvent::ProcessingInstruction { .. }
                | XmlEvent::Whitespace(_)
                | XmlEvent::Comment(_) => {}
                XmlEvent::CData(_) => {}
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
        })?;

        if let Some(ref text) = self.text {
            writer.write(XmlEvent::Characters(&text[..]))?;
        }
        for e in &self.children {
            e.write(writer)?;
        }

        writer.write(XmlEvent::EndElement { name: Some(name) })?;

        Ok(())
    }

    /// Find a single child of the current `Element`, given a predicate
    pub fn find_child<P>(&self, predicate: P) -> Option<&Element>
    where
        P: for<'r> Fn(&'r &Element) -> bool,
    {
        self.children.iter().find(predicate)
    }

    /// Filters the children of the current `Element`, given a predicate
    pub fn filter_children<P>(&self, predicate: P) -> Filter<Iter<Element>, P>
    where
        P: for<'r> Fn(&'r &Element) -> bool,
    {
        self.children.iter().filter(predicate)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let doc = ElementTree {
            root: Some(self.clone()),
            ..ElementTree::default()
        };
        let mut v = Vec::<u8>::new();
        doc.write_with(&mut v, false, "  ", true)?;
        let s = String::from_utf8(v).unwrap();
        f.write_str(&s[..])
    }
}
