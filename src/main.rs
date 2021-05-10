use baka::setting::read_plugin;
use baka::{
    parser::parse_args,
    setting::{read_project_setting, read_root_setting},
};
use std::env::args;

fn main() {
    // TEST
    // read_root_setting();
    // read_project_setting();
    // read_plugin();

    let args = args();
    let args_parsed = parse_args(args);
    println!(
        "Flags: {:?} Subcommand: {} Args: {:?}",
        args_parsed.baka_flags, args_parsed.subcommand, args_parsed.args
    );
}
