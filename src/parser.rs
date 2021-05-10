use std::env::Args;

pub struct BakaArgs {
    pub baka_flags: Vec<String>,
    pub subcommand: String,
    pub args: Vec<String>,
}

impl BakaArgs {
    fn new(baka_flags: Vec<String>, subcommand: String, args: Vec<String>) -> Self {
        Self {
            baka_flags,
            subcommand,
            args,
        }
    }

    pub fn parse_args(args: Args) -> Self {
        let mut baka_flags: Vec<String> = Vec::new();
        let mut subcommand = String::new();
        let mut argss: Vec<String> = Vec::new();
        let mut parsing_finished = 0;
        let mut get_flag_value = false;
        for (i, arg) in args.enumerate() {
            if i == 0 {
                continue;
            }
            if parsing_finished == 1 {
                argss.push(arg);
            } else if arg.starts_with("-") {
                get_flag_value = true;
                baka_flags.push(arg);
            } else {
                if get_flag_value {
                    baka_flags.push(arg);
                    get_flag_value = false
                } else if parsing_finished == 0 {
                    subcommand = arg;
                    parsing_finished += 1;
                }
            }
        }

        BakaArgs::new(baka_flags, subcommand, argss)
    }
}
