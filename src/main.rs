use crabyknife::commandline;

fn main() {
    let mut args = std::env::args();

    let program_name = args.next().expect("we should always has the program name");

    let subcommand = args
        .next()
        .unwrap_or_else(|| panic!("usage: {program_name} <subcommand>"));

    if let Err(err) = commandline::run(&subcommand, args) {
        eprint!("{err}");
        std::process::exit(1);
    }
}
