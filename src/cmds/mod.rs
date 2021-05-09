use clap::ArgMatches;

pub mod install;

pub fn match_subcommands(matches: ArgMatches) {
    match matches.subcommand() {
        ("install", Some(matches)) => {
            let args = if let Some(args) = matches.values_of("args") {
                args.collect::<Vec<&str>>().join(" ")
            } else {
                String::new()
            };

            let name = matches.value_of("package-name").unwrap();

            install::execute(name, args.as_str());
        }
        _ => {}
    }
}
