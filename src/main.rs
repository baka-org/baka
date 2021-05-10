use baka::setting::{read_project_setting, read_root_setting};
use baka::{parser::BakaArgs, setting::read_plugin};
use std::env::args;

fn main() {
    // TEST
    // read_root_setting();
    // read_project_setting();
    // read_plugin();

    let args = args();
    let args_parsed = BakaArgs::parse_args(args);
    println!(
        "Flags: {:?} Subcommand: {} Args: {:?}",
        args_parsed.baka_flags, args_parsed.subcommand, args_parsed.args
    );
}
