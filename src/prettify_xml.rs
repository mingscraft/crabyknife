use quick_xml::{events::Event, Reader};

/// Prettify a given raw(unprettified) xml text,
/// format it with identations and newlines.
///
/// # Example
/// ```
///
/// use crabyknife::prettify_xml::prettify_xml;
/// assert_eq!(prettify_xml("<root><child>text</child></root>").unwrap(), "<root>\n  <child>text</child>\n</root>");
///
/// ```
///
pub fn prettify_xml(unprettified_xml: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(unprettified_xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut output = String::new();
    let mut indent = 0;
    let indent_str = "  ";
    let mut child_is_text = false;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => {
                output.push_str(&"\n".repeat(1));
                output.push_str(&indent_str.repeat(indent));
                output.push('<');
                output.push_str(&String::from_utf8_lossy(e.name().as_ref()));
                for attr in e.attributes().with_checks(false) {
                    let attr = attr?;
                    output.push(' ');
                    output.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                    output.push_str("=\"");
                    output.push_str(&String::from_utf8_lossy(&attr.value));
                    output.push('"');
                }
                output.push('>');
                indent += 1;
            }
            Event::End(ref e) => {
                indent -= 1;

                // if the child of the current tag is `Text`,
                // we don't want to add newline before the closing tag.
                // For example:
                // we want:
                // <content>I am content</content>
                //
                // not:
                // <content>I am content
                // </content>
                //
                // But for other not closing tag, we want a newline before the closing tag.
                // <parent>
                //   <child />
                // </parent>
                if !child_is_text {
                    output.push_str(&"\n".repeat(1));
                    output.push_str(&indent_str.repeat(indent));
                }
                output.push_str("</");
                output.push_str(&String::from_utf8_lossy(e.name().as_ref()));
                output.push('>');
                child_is_text = false;
            }
            Event::Text(e) => {
                let text = e.into_inner();
                if !text.is_empty() {
                    output.push_str(&String::from_utf8_lossy(&text));
                }
                child_is_text = true;
            }
            Event::CData(e) => {
                output.push_str("<![CDATA[");
                output.push_str(&e.decode()?);
                output.push_str("]]>");
            }
            Event::Comment(e) => {
                output.push_str(&"\n".repeat(1));
                output.push_str(&indent_str.repeat(indent));
                output.push_str("<!--");
                output.push_str(&e.unescape()?);
                output.push_str("-->");
            }
            Event::Decl(e) => {
                output.push_str(&"\n".repeat(1));
                output.push_str("<?xml");

                output.push_str(" version=\"");
                output.push_str(std::str::from_utf8(&e.version()?)?);
                output.push('"');

                if let Some(encoding) = e.encoding() {
                    output.push_str(" encoding=\"");
                    output.push_str(std::str::from_utf8(&encoding?)?);
                    output.push('"');
                }

                if let Some(standalone) = e.standalone() {
                    output.push_str(" standalone=\"");
                    output.push_str(std::str::from_utf8(&standalone?)?);
                    output.push('"');
                }

                output.push_str("?>");
            }
            Event::Empty(e) => {
                output.push_str(&"\n".repeat(1));
                output.push_str(&indent_str.repeat(indent));
                output.push('<');
                output.push_str(&String::from_utf8_lossy(e.name().as_ref()));
                for attr in e.attributes().with_checks(false) {
                    let attr = attr?;
                    output.push(' ');
                    output.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                    output.push_str("=\"");
                    output.push_str(&String::from_utf8_lossy(&attr.value));
                    output.push('"');
                }
                output.push_str(" />");
            }
            Event::PI(e) => {
                output.push_str(&"\n".repeat(1));
                output.push_str(&indent_str.repeat(indent));
                output.push_str("<?");
                output.push_str(&String::from_utf8_lossy(e.target().as_ref()));
                for attr in e.attributes().with_checks(false) {
                    let attr = attr?;
                    output.push(' ');
                    output.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                    output.push_str("=\"");
                    output.push_str(&String::from_utf8_lossy(&attr.value));
                    output.push('"');
                }
                output.push_str("?>");
            }
            Event::DocType(e) => {
                output.push('\n');
                output.push_str(&indent_str.repeat(indent));
                output.push_str("<!DOCTYPE ");
                output.push_str(&e.unescape()?);
                output.push('>');
            }
            Event::Eof => break,
        }

        buf.clear();
    }

    Ok(output.trim_start().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_element() {
        let input = "<root></root>";
        let expected = "<root>\n</root>";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_elements() {
        let input = "<root><child>text</child></root>";
        let expected = "<root>\n  <child>text</child>\n</root>";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_attributes() {
        let input = r#"<root><item id="1" value="x"/></root>"#;
        let expected = "<root>\n  <item id=\"1\" value=\"x\" />\n</root>";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_comment_and_cdata() {
        let input = r#"<root><!--comment--><![CDATA[some <xml>]]></root>"#;
        let expected = "<root>\n  <!--comment--><![CDATA[some <xml>]]>\n</root>";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_doctype() {
        let input = r#"<!DOCTYPE note><note><to>Tove</to></note>"#;
        let expected = "<!DOCTYPE note>\n<note>\n  <to>Tove</to>\n</note>";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_xml_declaration() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?><root/>"#;
        let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root />";
        let result = prettify_xml(input).unwrap();
        assert_eq!(result, expected);
    }
}
