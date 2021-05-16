use std::process::{Child, Command};

use crate::{parser::BakaArgs, plugins::plugins};

pub fn match_baka_flags(baka: BakaArgs) {
    match baka.baka_flags() {
        // Not found .baka.[json, toml, yaml]
        ("-p", Some(_)) => unimplemented!(),
        ("-l", Some(_)) => unimplemented!(),

        // Found .baka.[json, toml, yaml]
        (_, Some(_)) => unimplemented!(),

        (_, _) => match_subcommand(baka),
    }
}

fn match_subcommand(baka: BakaArgs) {
    match baka.subcommand() {
        ("plugin", Some(plugin)) => plugin_commands(plugin),
        ("help", Some(_)) => println!("{}", include_str!("../../res/HELP")),
        ("version", Some(_)) => println!("{}", include_str!("../../res/VERSION")),
        (_, _) => {}
    }
}

fn plugin_commands(plugin: Vec<String>) {
    if plugin.is_empty() {
        return;
    }

    match plugin[0].as_str() {
        "add" => unimplemented!(),
        "remove" => unimplemented!(),
        "list" => {
            println!("Plugin list:");

            for plugin in plugins() {
                println!(
                    " ㄴname: {}   version: {}",
                    plugin.settings.name, plugin.settings.version
                );
            }
        }
        _ => {}
    }
}

fn _command_output(program_name: &str, subcommand: &str, args: Vec<String>) -> Child {
    Command::new(program_name)
        .arg(subcommand)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("{} command failed to start", program_name))
}
