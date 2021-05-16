use std::env::{self, args};

use crate::{
    parser::BakaArgs,
    plugins::plugins,
    setting::{project, root},
};

pub mod commands;
pub mod parser;
pub mod plugins;
pub mod setting;

pub fn debug() {
    let root = root();
    let project = project();
    let plugins = plugins();

    println!(
        "Settings:\nRootSetting: {:?}\nProjectSetting: {:?}\nPlugins: {:?}\n",
        root, project, plugins
    );

    let args_parsed = BakaArgs::parse_args(args());
    println!(
        "Parser:\nFlags: {:?} Subcommand: {:?} Args: {:?}\n",
        args_parsed.baka_flags, args_parsed.subcommand, args_parsed.args
    );
    println!(
        "Env:\nbaka_plugins: {}\nbaka_root_setting: {}",
        env::var("baka_plugins").unwrap(),
        env::var("baka_root_setting").unwrap()
    );
}
