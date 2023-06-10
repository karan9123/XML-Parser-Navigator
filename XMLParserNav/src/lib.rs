#![allow(warnings)]
mod element;
mod errors;
mod translator;
mod tree;
mod parser;
mod tokens;

#[cfg(test)]
mod test {

    use crate::parser::Token;
    use crate::translator::{Bridge, ReturnEnum};
    use crate::tree::ElementTree;

    #[test]
    fn test_child_query() {
        let doc_raw = r#"
    <?xml version="1.1" encoding="UTF-8"?>
    <catalog> catalog test
       <book id="first" name="Rahul" xmlns:first="Some_Prefix">
          <first:author>Gambardella, Matthew</first:author>this is book1 text
          <first:title>XML Developer's Guide</first:title>
          <first:genre>Computer</first:genre>
          <first:price>44.95</first:price>
          <first:publish_date>2000-10-01</first:publish_date>
          <description>An in-depth look at creating applications
          with XML.</description>
       </book> This is again, a text in catalog
       <book id="second" xmlns:h="Some_Prefix">
       This is book2 text
          <author>Ralls, Kim</author>
          <title>Midnight Rain</title>
          <genre>Fantasy</genre>
          <h:price>5.95</h:price>
          <publish_date>2000-12-16</publish_date>
          <description>A former architect battles corporate zombies,
          an evil sorceress, and her own childhood to become queen
          of the world.</description>
        </book>
    </catalog>
    "#;

        let doc = ElementTree::parse(doc_raw.as_bytes()).unwrap();
        let query: String = "//title[2]".to_string();
        let mut bridge1 = Bridge::new(doc, query);
        bridge1.token_step_filler();
        let result: Result<Vec<crate::translator::ReturnEnum>, crate::errors::Error> = bridge1.produce();
        let k = result.unwrap();
        match k[0]{
            ReturnEnum::ElementNode(t) => {
                // let k  =t.text.as_ref().unwrap().split_whitespace()
                assert_eq!("Midnight Rain".to_string(),t.text.as_ref().unwrap().to_string());
            }
            ReturnEnum::ElementName(_) => { }
        }
    }
}