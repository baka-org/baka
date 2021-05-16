use std::env::Args;

// baka [baka_flags] [subcommand] [args...]
pub struct BakaArgs {
    pub baka_flags: Option<Vec<String>>,
    pub subcommand: Option<String>,
    pub args: Option<Vec<String>>,
}

impl BakaArgs {
    fn new(
        baka_flags: Option<Vec<String>>,
        subcommand: Option<String>,
        args: Option<Vec<String>>,
    ) -> Self {
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
            } else if i == 1 && arg.starts_with('-') {
                get_flag_value = true;
                baka_flags.push(arg);
            } else if get_flag_value {
                baka_flags.push(arg);
                get_flag_value = false
            } else if parsing_finished == 0 {
                subcommand = arg;
                parsing_finished += 1;
            }
        }

        let baka_flags = if baka_flags.is_empty() {
            None
        } else {
            Some(baka_flags)
        };

        let subcommand = if subcommand.is_empty() {
            None
        } else {
            Some(subcommand)
        };

        let argss = if argss.is_empty() { None } else { Some(argss) };

        BakaArgs::new(baka_flags, subcommand, argss)
    }

    pub fn baka_flags(&self) -> (&str, Option<&Self>) {
        self.baka_flags
            .as_ref()
            .map_or(("", None), |f| (f[0].as_str(), Some(self)))
    }

    pub fn subcommand(&self) -> (&str, Option<Vec<String>>) {
        self.subcommand.as_ref().map_or(("", None), |f| {
            (
                f.as_str(),
                Some(self.args.as_ref().unwrap_or(&vec![String::new()]).to_vec()),
            )
        })
    }
}
