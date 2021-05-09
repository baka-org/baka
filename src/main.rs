use baka::setting::read_plugin;
use baka::{
    cmds::{install::install_command, match_subcommands},
    setting::{read_project_setting, read_root_setting},
};
use clap::{App, AppSettings, Arg};

fn main() {
    // TEST
    read_root_setting();
    read_project_setting();
    read_plugin();

    let app = App::new("baka")
        .bin_name("baka")
        .version("1.0")
        .setting(AppSettings::DisableHelpFlags)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("package-manager-name")
                .short("p")
                .long("package-manager-name")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("language-name")
                .short("l")
                .long("language-name")
                .required(false)
                .takes_value(true),
        )
        .subcommand(install_command())
        .get_matches();

    match_subcommands(app);
}
