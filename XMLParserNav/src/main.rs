use crate::errors::Error;
use crate::translator::{Bridge, ReturnEnum};
use crate::tree::ElementTree;


mod tree;
mod element;
mod errors;
mod parser;
mod translator;
mod tokens;

fn main() {
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
       <book id="second" xmlns:h="Some_Prefix">This is book2 text
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

    let query1: String = "//book[2]".to_string();
    let _query2: String = "childcatalog/book[@id=first]".to_string();
    let _query3: String = "//namespaceprice[1]".to_string();

    let mut bridge1 = Bridge::new(doc, query1);
    bridge1.token_step_filler();
    let result: Result<Vec<ReturnEnum>, Error> = bridge1.produce();

    for ret in result.unwrap() {
        match ret {
            ReturnEnum::ElementNode(l) => {
                println!("{}", l.text.as_ref().unwrap());
            }
            ReturnEnum::ElementName(l) => {
                println!("{}", l);
            }
        }
    }


    // println!("{}", l);
    /*if k.is_ok(){
        for i in k.unwrap().iter(){
            println!("tag: {}", i.namespace.as_ref().unwrap());
        }
    }else{
        println!("{:?}", k.unwrap_err());
    }*/


    /* let children = &catalog.children;
     for child in children{
         println!("{}", child.attributes.get("id").unwrap());
     }*/


    // let p = |el| {el.tag == "book" & el.attributes.get("id").unwrap().contains("01")};
    // let k = catalog.find_child(p);

    // let book2 = catalog.find_child(|el| el.tag == "book" && el.attributes.get("id").unwrap().contains("02")).unwrap().clone();
    // let book2_price = book2.find_child(|tag| tag.tag == "price").unwrap().clone();
    // println!("{} [{:?}] = {}", book2.tag, book2.attributes, if book2.text.is_none(){"".to_string()}else{book2.text.unwrap()} );
    // println!("{}: {:?}, Price: {} ",book2_price.namespace.unwrap(), book2.attributes, book2_price.text.unwrap() );
    //
    // let mut bridge1 = Bridge::new(doc, "//catalog//book[0]...".to_string());
    //
    // // let xpath = Token::new( "///.(".to_string());
    //
    // bridge1.token_step_filler();
    //
    // for i in bridge1.token_steps{
    //     println!("{}", i)
    // }
}