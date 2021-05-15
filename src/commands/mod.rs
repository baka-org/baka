use std::process::{Child, Command};

use crate::parser::BakaArgs;

pub fn match_subcommands(baka: BakaArgs) {}

fn command_output(pacakge_name: &str, subcommand: &str, args: Vec<String>) -> Child {
    Command::new(pacakge_name)
        .arg(subcommand)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("{} command failed to start", pacakge_name))
}
