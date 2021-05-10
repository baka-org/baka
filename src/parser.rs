// pub struct Parser {
//    pub args: Vec<String>,
//    pub sub_commands: Vec<SubCommand>,
// }

// pub struct SubCommand {
//     pub name: String,
//     pub help: String,
//     pub description: String,
// }

use std::env::Args;

pub struct BakaArgs {
  pub baka_flags: Vec<String>,
  pub subcommand: String,
  pub args: Vec<String>,
}

pub fn parse_args(args: Args) -> BakaArgs {
  let mut baka_flags: Vec<String> = vec![];
  let mut subcommand = String::from("");
  let mut argss: Vec<String> = vec![];
  let mut parsing_finished = 0;
  let mut get_flag_value = false;
  for (i, arg) in args.enumerate() {
    if i == 0 {
      continue;
    }
    if parsing_finished == 2 {
      argss.push(arg);
    } else if arg.starts_with("-") {
      get_flag_value = true;
      baka_flags.push(arg);
    } else {
      if get_flag_value {
        baka_flags.push(arg);
        get_flag_value = false
      } else if parsing_finished == 0 {
        parsing_finished += 1;
        subcommand = arg;
        parsing_finished += 1;
      }
    }
  }

  BakaArgs {
    baka_flags: baka_flags,
    subcommand: subcommand,
    args: argss,
  }
}
