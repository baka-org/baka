use baka::{commands::match_baka_flags, parser::BakaArgs, setting::init};
use std::env::args;

fn main() {
    // INIT
    init();

    match_baka_flags(BakaArgs::parse_args(args()));
}
