use crate::prettify_xml;

pub enum Subcommands {
    PrettifyXml,
    NewUuid,
}

impl std::str::FromStr for Subcommands {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prettify-xml" => Ok(Self::PrettifyXml),
            "new-uuid" => Ok(Self::NewUuid),
            _ => Err("support subcommands"),
        }
    }
}

pub fn run(
    subcommand: &str,
    remaining_args: std::env::Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let subcommand: Subcommands = subcommand.parse()?;

    match subcommand {
        Subcommands::PrettifyXml => handle_prettify_xml(remaining_args),
        Subcommands::NewUuid => handle_new_uuid(),
    }
}

fn handle_prettify_xml(
    mut remaining_args: std::env::Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let xml = remaining_args
        .next()
        .expect("usage: crabyknif prettify-xml <unprettified xml>");

    let prettified = prettify_xml::prettify_xml(&xml)?;
    println!("{prettified}");
    Ok(())
}

fn handle_new_uuid() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", uuid::Uuid::new_v4());
    Ok(())
}
