use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use log::info;
use xmltree::{Element, XMLNode};
use crate::lib::core::box_view::Padding;
use crate::lib::core::view::View;

pub fn parse() {
    create_from_xml_file("main.xml");
}

pub fn create_from_xml_file(path: &str) -> Rc<RefCell<Box<dyn View>>> {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file); // Buffering is important for performance

    let names_element: Element = Element::parse(file).unwrap();
    println!("{:#?}", names_element);
    return create_from_xml_element(names_element);
}

pub fn create_from_xml_element(element: Element) -> Rc<RefCell<Box<dyn View>>> {
    info!("create_from_xml_element: {}", element.name);
    for child in element.children {
        match child {
            XMLNode::Element(sub_element) => {
                handle_xml_element(sub_element);
            }
            XMLNode::Comment(_) => {}
            XMLNode::CData(_) => {}
            XMLNode::Text(_) => {}
            XMLNode::ProcessingInstruction(_, _) => {}
        }
    }
    Padding::create()
}

pub fn handle_xml_element(element: Element) {
    info!("handle_xml_element: {}", element.name);
    create_from_xml_element(element);
}