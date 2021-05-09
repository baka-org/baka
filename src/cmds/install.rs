use clap::{App, Arg, SubCommand};

pub fn install_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("install")
        .arg(
            Arg::with_name("package-name")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("args")
                .required(false)
                .takes_value(true)
                .raw(true),
        )
}

pub fn execute(name: &str, args: &str) {
    println!("{}, {}", name, args);
}
