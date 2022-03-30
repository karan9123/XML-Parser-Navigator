use std::io::Read;
use xml::common::XmlVersion;
use crate::element::{Element, ElementBuilder};
use xml::reader::{EventReader, XmlEvent};

pub struct ElementTree {
    root: Option<Element>,
}

impl default for ElementTree {
    fn default() -> Self {
        ElementTree {
            root: None
        }
    }
}

impl ElementTree {
    ///Method creates a new `ElementTree`
    fn new() -> ElementTree {
        ElementTree {
            ..ElementTree::default()
        }
    }

    ///Method creates a new `ElementTree` with root from ElementBuilder
    fn new_with_root(root: ElementBuilder) -> ElementTree {
        ElementTree {
            root: Some(root.element())
        }
    }

    ///Returns reference to the root element
    fn get_root(&self) -> &Element {
        if self.root.is_some() {
            return &self.root.unwrap();
        }
        &None
    }

    ///Returns mutable reference to the root element
    fn get_root_mut(&self) -> &mut Element {
        if self.root.is_some() {
            return &mut self.root.unwrap();
        }
        &mut None
    }

    //Add set root here(if needed)

    ///Load external XML document into element tree
    fn parse<T: Read>(read: T) -> Result<ElementTree, TreexmlError> {
        let mut parser = EventReader::new(read);
        let mut tree = ElementTree::new();

        loop {
            let event = parser.next();
            match event {
                XmlEvent::StartDocument {
                    version, encoding, ..
                } => {
                    tree.version = XmlVersion::from(version);
                    tree.encoding = encoding;
                }
                XmlEvent::StartElement { name, attributes, .. } => {
                    //Start

                }
            }
        }
    }
}


















