use minidom::Element;
use minidom::Node;

pub fn get_attr(el: &Element, attr: &str) -> Option<String> {
    match el.attr(attr) {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

pub fn get_attr_default<T: std::str::FromStr>(
    el: &Element,
    name: &str,
    default: T,
) -> Result<T, String> {
    let value = match el.attr(name) {
        Some(val) => match val.parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(format!(
                    "Element '{}' attribute '{}' can't parse value '{}'.",
                    el.name(),
                    name,
                    val
                ))
            }
        },
        None => default,
    };
    Ok(value)
}

pub fn get_attr_default_options(
    el: &Element,
    name: &str,
    default: String,
    options: Vec<&str>,
) -> Result<String, String> {
    let val = get_attr_default(el, name, default)?;
    if options.contains(&&val[..]) {
        return Ok(val);
    }
    Err(format!(
        "Element '{}' attribute '{}'='{}', but expected one of {:?}",
        el.name(),
        name,
        val,
        options
    ))
}

pub fn require_attr(el: &Element, attr: &str) -> Result<String, String> {
    match el.attr(attr) {
        Some(val) => Ok(val.to_string()),
        None => Err(format!(
            "Element '{}' doesn't have required '{}' attribute",
            el.name(),
            attr
        )),
    }
}

pub fn require_attr_options(
    el: &Element,
    attr: &str,
    options: Vec<&str>,
) -> Result<String, String> {
    let val = require_attr(el, attr)?;
    if options.contains(&&val[..]) {
        return Ok(val);
    }
    Err(format!(
        "Element '{}' attribute '{}'='{}', but expected one of {:?}",
        el.name(),
        attr,
        val,
        options
    ))
}

pub fn html_to_string(el: &Element) -> String {
    let mut text = String::new();
    for node in el.nodes() {
        match node {
            Node::Text(x) => text.push_str(&x.replace("\n", " ")),
            Node::Element(x) => match x.name() {
                "br" => text.push('\n'),
                _ => text.push_str(&x.text().replace("\n", " ")),
            },
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attr() {
        let el: Element = "<person xmlns=\"people\" name=\"John\">".parse().unwrap();
        assert_eq!(get_attr(&el, "name"), Some(String::from("John")));
    }

    #[test]
    fn test_get_none() {
        let el: Element = "<person xmlns=\"people\" login=\"jdoe\">".parse().unwrap();
        assert_eq!(get_attr(&el, "name"), None);
    }

    #[test]
    fn test_get_attr_default() {
        let el1 = Element::builder("person", "ns").attr("age", "24").build();
        let val1 = get_attr_default(&el1, "age", 17);
        assert_eq!(val1, Ok(24));
        let el2 = Element::builder("person", "ns").build();
        let val2 = get_attr_default(&el2, "age", 17);
        assert_eq!(val2, Ok(17));
        let el3 = Element::builder("person", "ns").attr("age", "24").build();
        let val3 = get_attr_default(&el3, "age", String::from("17"));
        assert_eq!(val3, Ok(String::from("24")));
        let el4 = Element::builder("person", "ns").build();
        let val4 = get_attr_default(&el4, "age", String::from("17"));
        assert_eq!(val4, Ok(String::from("17")));
    }

    #[test]
    fn test_require_attr() {
        let el: Element = "<person xmlns=\"people\" name=\"John\">".parse().unwrap();
        assert_eq!(require_attr(&el, "name"), Ok(String::from("John")));
    }

    #[test]
    fn test_require_attr_missing() {
        let el: Element = "<person xmlns=\"people\">".parse().unwrap();
        assert_eq!(
            require_attr(&el, "name"),
            Err(String::from(
                "Element 'person' doesn't have required 'name' attribute"
            ))
        );
    }

    #[test]
    fn test_require_attr_options() {
        let el: Element = "<person xmlns=\"people\" name=\"John\">".parse().unwrap();
        assert_eq!(
            require_attr_options(&el, "name", vec!["John", "Peter"]),
            Ok(String::from("John"))
        );
    }

    #[test]
    fn test_require_attr_options_wrong() {
        let el: Element = "<person xmlns=\"people\" name=\"Albert\">".parse().unwrap();
        assert_eq!(require_attr_options(&el, "name", vec!["John", "Peter"]), Err(String::from("Element 'person' attribute 'name'='Albert', but expected one of [\"John\", \"Peter\"]")));
    }

    #[test]
    fn test_html_to_string() {
        let el: Element =
            "<description xmlns=\"xccdf\">We are\nthe <em>best</em> project!</description>"
                .parse()
                .unwrap();
        assert_eq!(
            html_to_string(&el),
            String::from("We are the best project!")
        );
        let el: Element =
        "<description xmlns=\"xccdf\">Open it<br/>and then close it <b>quickly</b>.</description>"
            .parse()
            .unwrap();
        assert_eq!(
            html_to_string(&el),
            String::from("Open it\nand then close it quickly.")
        );
    }
}
