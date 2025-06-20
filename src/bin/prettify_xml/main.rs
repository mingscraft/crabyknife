use crabyknife::prettify_xml;

fn main() {
    let xml = std::env::args()
        .nth(1)
        .expect("usage: prettify-xml <unprettified xml>");

    match prettify_xml::prettify_xml(&xml) {
        Ok(prettified) => println!("{prettified}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
