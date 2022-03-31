use crate::tree::ElementTree;

mod tree;
mod element;
mod version;
mod errors;

fn main() {

    let doc_raw = r#"
    <?xml version="1.1" encoding="UTF-8"?>
    <catalog>
       <book id="bk101" name="Rahul" xmlns:f="Some_Prefix">
          <f:author>Gambardella, Matthew</f:author>
          this is test text
          <f:title>XML Developer's Guide</f:title>
          <f:genre>Computer</f:genre>
          <f:price>44.95</f:price>
          <f:publish_date>2000-10-01</f:publish_date>
          <description>An in-depth look at creating applications
          with XML.</description>
       </book> This is again, a text
       <book id="bk102" xmlns:f="Some_Prefix">This is text
          <author>Ralls, Kim</author>
          <title>Midnight Rain</title>
          <genre>Fantasy</genre>
          <f:price>5.95</f:price>
          <publish_date>2000-12-16</publish_date>
          <description>A former architect battles corporate zombies,
          an evil sorceress, and her own childhood to become queen
          of the world.</description>
        </book>
    </catalog>
    "#;

    let doc = ElementTree::parse(doc_raw.as_bytes()).unwrap();
    let catalog = doc.root.unwrap();

   /* let children = &catalog.children;
    for child in children{
        println!("{}", child.attributes.get("id").unwrap());
    }*/


    let book2 = catalog.find_child(|el| el.tag == "book" && el.attributes.get("id").unwrap().contains("02")).unwrap().clone();
    let book2_price = book2.find_child(|tag| tag.tag == "price").unwrap().clone();
    println!("{} [{:?}] = {}", book2.tag, book2.attributes, if book2.text.is_none(){"".to_string()}else{book2.text.unwrap()} );
    println!("{}: {:?}, Price: {} ",book2_price.namespace.unwrap(), book2.attributes, book2_price.text.unwrap() );

}
