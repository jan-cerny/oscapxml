use minidom::quick_xml;
use minidom::Element;
use oscapxml::sds;

#[test]
fn test_simple() {
    let filepath = "data/simple.xml";
    let mut reader = quick_xml::Reader::from_file(filepath).expect("Failed to open the input file");
    let root = Element::from_reader(&mut reader).unwrap();
    let result = sds::DataStreamCollection::from_xml(&root);
    assert!(result.is_ok());
}
