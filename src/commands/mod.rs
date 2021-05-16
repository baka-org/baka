use std::process::{Child, Command};

use crate::parser::BakaArgs;

pub fn match_baka_flags(baka: BakaArgs) {
    match baka.baka_flags() {
        // Not found .baka.[json, toml, yaml]
        ("-p", Some(_)) => {}
        ("-l", Some(_)) => {}

        // Found .baka.[json, toml, yaml]
        (_, Some(_)) => {}

        (_, _) => match_subcommands(baka),
    }
}

fn match_subcommands(baka: BakaArgs) {
    match baka.subcommand() {
        ("plugin", Some(plugin)) => plugin_commands(plugin),
        ("help", Some(_)) => println!("{}", include_str!("../../res/HELP")),
        ("version", Some(_)) => println!("{}", include_str!("../../res/VERSION")),
        (_, _) => {}
    }
}

fn plugin_commands(_plugin: Vec<String>) {}

fn _command_output(pacakge_name: &str, subcommand: &str, args: Vec<String>) -> Child {
    Command::new(pacakge_name)
        .arg(subcommand)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("{} command failed to start", pacakge_name))
}
