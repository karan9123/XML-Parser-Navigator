use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use xml::common::XmlVersion;
use crate::element::{Element, ElementBuilder};
use xml::reader::{EventReader, XmlEvent};
use xml::writer::EmitterConfig;
use xml::writer::XmlEvent as writer_XmlEvent;
use crate::errors::XmlErrors;

pub struct ElementTree {
    pub(crate) root: Option<Element>,
    pub(crate) version: XmlVersion,
    pub(crate) encoding: String,
}

impl Default for ElementTree {
    fn default() -> Self {
        ElementTree {
            root: None,
            version: XmlVersion::Version10,
            encoding: "UTF-8".to_string(),
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
            root: Some(root.element()),
            ..Self::default()
        }
    }

    ///Returns true if root is None and false if the ElementTree is not empty
    fn check_blank_tree(&self)->bool{
        return self.root.is_none()
    }


    ///Returns reference to the root element
    fn get_root(&self) -> Option<Element> {
        if !self.check_blank_tree() {
            return Some(self.root.as_ref().unwrap().clone());
        }
        None
    }


    /*///Returns mutable reference to the root element
    fn get_root_mut(&self) -> Option<&mut Element> {
        if self.root.is_some() {
            return Some(&mut self.root.as_ref().unwrap().clone());
        }
        None
    }
*/
    //Add set root here(if needed)

    ///Load external XML document into element tree
    pub(crate) fn parse<T: Read>(read: T) -> Result<ElementTree, XmlErrors> {
        let mut parser = EventReader::new(read);
        let mut tree = ElementTree::new();

        loop {
            let event = parser.next()?;
            match event {
                /*  XmlEvent::StartDocument {
                      version, encoding, ..
                  } => {
                      tree.version = XmlVersion::from(version);
                      tree.encoding = encoding;
                  },*/
                XmlEvent::StartElement { name, attributes, .. } => {
                    let mut attribute = HashMap::new();
                    for att in attributes {
                        let attr_name = match att.name.prefix {
                            Some(p) => { format!("{}:{}", p, att.name.local_name) }
                            None => { att.name.local_name }
                        };
                        attribute.insert(attr_name, att.value);
                    }
                    let mut root = Element {
                        namespace: name.namespace,
                        tag: name.local_name,
                        attributes: attribute,
                        ..Element::default()
                    };
                    root.parse(&mut parser)?;
                    tree.root = Some(root);
                }
                XmlEvent::EndDocument => break,
                _ => {}
            }
        }
        Ok(tree)
    }

    pub fn write<T: Write>(&self, mut t: &mut T) -> Result<(), XmlErrors> {
        self.write_with(&mut t, true, " ", true)
    }

    ///Writes to `t`
    pub(crate) fn write_with<T: Write>(&self, t: &mut T, doc_decl: bool, indnt_str: &'static str, indent: bool) //Removed 'static from indnt_str
                            -> Result<(), XmlErrors> {


        let mut writer = EmitterConfig::new().perform_indent(indent)
            .write_document_declaration(doc_decl)
            .indent_string(indnt_str)
            .create_writer(t);

        if doc_decl {
            writer.write(writer_XmlEvent::StartDocument {
                version: self.version.into(),
                encoding: Some(&self.encoding),
                standalone: None,
            })?;
        }
        if let Some(ref e) = self.root {
            e.write(&mut writer)?;
        }
        Ok(())
    }
}


impl fmt::Display for ElementTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec = Vec::<u8>::new();
        self.write(&mut vec)?;
        let strng = String::from_utf8(vec).unwrap();
        f.write_str(&strng[..])
    }
}
















