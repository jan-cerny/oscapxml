use minidom::Element;

const SCAP12_NS: &str = "http://scap.nist.gov/schema/scap/source/1.2";
const DSIG_NS: &str = "http://scap.nist.gov/schema/xml-dsig/1.0";
const CAT_NS: &str = "urn:oasis:names:tc:entity:xmlns:xml:catalog";

use crate::utils::*;
use crate::xccdf;

#[derive(Debug)]
pub struct DataStreamCollection {
    id: String,
    schematron_version: String,
    data_streams: Vec<DataStream>,
    components: Vec<Component>,
    extended_components: Vec<ExtendedComponent>,
    signatures: Vec<Signature>,
}

impl DataStreamCollection {
    pub fn from_xml(root: &Element) -> Result<DataStreamCollection, String> {
        if root.ns() != SCAP12_NS {
            return Err(format!(
                "Wrong namespace '{}', expected '{}",
                root.ns(),
                SCAP12_NS
            ));
        }
        let id = require_attr(root, "id")?;
        let schematron_version = require_attr(root, "schematron-version")?;

        let mut data_streams = Vec::new();
        let mut components = Vec::new();
        let mut extended_components = Vec::new();
        let mut signatures = Vec::new();
        for child in root.children() {
            if child.is("data-stream", SCAP12_NS) {
                let data_stream = DataStream::from_xml(child)?;
                data_streams.push(data_stream);
            } else if child.is("component", SCAP12_NS) {
                let component = Component::from_xml(child)?;
                components.push(component);
            } else if child.is("extended-component", SCAP12_NS) {
                let component = ExtendedComponent::from_xml(child)?;
                extended_components.push(component);
            } else if child.is("Signature", DSIG_NS) {
                let signature = Signature::from_xml(child)?;
                signatures.push(signature);
            }
        }
        if data_streams.len() < 1 {
            return Err(String::from("The 'data-stream-collection' element needs to have at least 1 child 'data-stream' element."));
        }
        if components.len() < 1 {
            return Err(String::from("The 'data-stream-collection' element needs to have at least 1 child 'component' element."));
        }
        Ok(DataStreamCollection {
            id,
            schematron_version,
            data_streams,
            components,
            extended_components,
            signatures,
        })
    }

    pub fn print_information(&self) {
        println!("Document type: SCAP Source Data Stream");
        for ds in self.data_streams.iter() {
            println!("Stream: {}", ds.id);
            println!();
            println!("Checklists:");
            for checklist in ds.checklists.iter() {
                println!("Ref-Id: {}", checklist.id);
                if !checklist.href.starts_with("#") {
                    println!("Remote checklists aren't supported by this tool");
                    continue;
                }
                for component in self.components.iter() {
                    if &checklist.href[1..] == component.id {
                        println!("Component ID: {}", component.id);
                        let content = &component.content;
                        match content {
                            ComponentContent::XCCDFBenchmark(benchmark) => {
                                benchmark.print_information()
                            }
                            _ => panic!("The component isn't a XCCDF benchmark"),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct DataStream {
    id: String,
    use_case: String,
    scap_version: String,
    timestamp: Option<String>,
    dictionaries: Vec<ComponentRef>,
    checklists: Vec<ComponentRef>,
    checks: Vec<ComponentRef>,
    extended_components: Vec<ComponentRef>,
}

impl DataStream {
    pub fn from_xml(el: &Element) -> Result<DataStream, String> {
        let id = require_attr(el, "id")?;
        let use_case = require_attr_options(
            el,
            "use-case",
            vec!["CONFIGURATION", "VULNERABILITY", "INVENTORY", "OTHER"],
        )?;
        let scap_version =
            require_attr_options(el, "scap-version", vec!["1.0", "1.1", "1.2", "1.3"])?;
        let timestamp = get_attr(el, "timestamp");
        let dictionaries = DataStream::get_component_ref_vec(el, "dictionaries")?;
        let checklists = DataStream::get_component_ref_vec(el, "checklists")?;
        let checks = DataStream::get_component_ref_vec(el, "checks")?;
        let extended_components = DataStream::get_component_ref_vec(el, "extended-components")?;
        Ok(DataStream {
            id,
            use_case,
            scap_version,
            timestamp,
            dictionaries,
            checklists,
            checks,
            extended_components,
        })
    }

    fn get_component_ref_vec(
        data_stream_el: &Element,
        component_name: &str,
    ) -> Result<Vec<ComponentRef>, String> {
        let mut component_refs = Vec::new();
        if let Some(component_el) = data_stream_el.get_child(component_name, SCAP12_NS) {
            for component_ref_el in component_el.children() {
                let component_ref = ComponentRef::from_xml(component_ref_el)?;
                component_refs.push(component_ref);
            }
        }
        Ok(component_refs)
    }
}

#[derive(Debug)]
enum ComponentContent {
    XCCDFBenchmark(xccdf::Benchmark),
    NotImplemented,
}

#[derive(Debug)]
struct Component {
    id: String,
    timestamp: String,
    component_name: String,
    component_ns: String,
    content: ComponentContent,
}

impl Component {
    fn from_xml(el: &Element) -> Result<Component, String> {
        let id = require_attr(el, "id")?;
        let timestamp = require_attr(el, "timestamp")?;
        if let Some(component) = el.children().next() {
            let component_name = component.name().to_string();
            let component_ns = component.ns();
            let mut content = ComponentContent::NotImplemented;
            if component_ns == xccdf::XCCDF12_NS && component_name == "Benchmark" {
                content = ComponentContent::XCCDFBenchmark(xccdf::Benchmark::from_xml(component)?);
            }
            Ok(Component {
                id,
                timestamp,
                component_name,
                component_ns,
                content,
            })
        } else {
            Err(format!("component '{}' doesn't have any child element", id))
        }
    }
}

#[derive(Debug)]
struct ExtendedComponent {
    id: String,
    timestamp: String,
}

impl ExtendedComponent {
    fn from_xml(el: &Element) -> Result<ExtendedComponent, String> {
        let id = require_attr(el, "id")?;
        let timestamp = require_attr(el, "timestamp")?;
        Ok(ExtendedComponent { id, timestamp })
    }
}

#[derive(Debug)]
struct Signature {
    id: String,
}

impl Signature {
    fn from_xml(el: &Element) -> Result<Signature, String> {
        let id = require_attr(el, "id")?;
        Ok(Signature { id })
    }
}

#[derive(Debug)]
struct ComponentRef {
    id: String,
    type_: Option<String>,
    href: String,
    catalog: Option<Catalog>,
}

impl ComponentRef {
    fn from_xml(el: &Element) -> Result<ComponentRef, String> {
        if !el.is("component-ref", SCAP12_NS) {
            return Err(format!("Unexpected element '{}'", el.name()));
        }
        let id = require_attr(el, "id")?;
        let type_ = get_attr(el, "xlink:type");
        let href = require_attr(el, "xlink:href")?;
        let catalog = match el.get_child("catalog", CAT_NS) {
            Some(catalog_el) => Some(Catalog::from_xml(catalog_el)?),
            _ => None,
        };
        Ok(ComponentRef {
            id,
            type_,
            href,
            catalog,
        })
    }
}

#[derive(Debug)]
struct Catalog {
    uris: Vec<CatURI>,
    rewrite_uris: Vec<RewriteURI>,
}

impl Catalog {
    fn from_xml(el: &Element) -> Result<Catalog, String> {
        if !el.is("catalog", CAT_NS) {
            return Err(format!("Unexpected element '{}'", el.name()));
        }
        let mut uris = Vec::new();
        let mut rewrite_uris = Vec::new();
        for child in el.children() {
            if child.is("uri", CAT_NS) {
                uris.push(CatURI::from_xml(child)?);
            } else if child.is("rewriteURI", CAT_NS) {
                rewrite_uris.push(RewriteURI::from_xml(child)?);
            } else {
                return Err(format!(
                    "Unexpected element '{}', expected either 'uri' or 'rewriteURI'",
                    el.name()
                ));
            }
        }

        Ok(Catalog { uris, rewrite_uris })
    }
}

#[derive(Debug)]
struct CatURI {
    name: String,
    uri: String,
}

impl CatURI {
    fn from_xml(el: &Element) -> Result<CatURI, String> {
        let name = require_attr(el, "name")?;
        let uri = require_attr(el, "uri")?;
        Ok(CatURI { name, uri })
    }
}

#[derive(Debug)]
struct RewriteURI {
    uri_start_string: String,
    rewrite_prefix: String,
}

impl RewriteURI {
    fn from_xml(el: &Element) -> Result<RewriteURI, String> {
        let uri_start_string = require_attr(el, "uriStartString")?;
        let rewrite_prefix = require_attr(el, "rewritePrefix")?;
        Ok(RewriteURI {
            uri_start_string,
            rewrite_prefix,
        })
    }
}
