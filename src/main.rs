use baka::parser::BakaArgs;
use baka::setting::init_config;
use std::env::{self, args};

fn main() {
    // TEST
    // read_root_setting();
    // read_project_setting();
    // read_plugin();

    let args = args();
    let args_parsed = BakaArgs::parse_args(args);
    println!(
        "Flags: {:?} Subcommand: {:?} Args: {:?}",
        args_parsed.baka_flags, args_parsed.subcommand, args_parsed.args
    );
    init_config();
    println!(
        "{}, {}",
        env::var("baka_plugins").unwrap(),
        env::var("baka_root_setting").unwrap()
    )
}
