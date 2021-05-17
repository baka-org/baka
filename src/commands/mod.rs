use std::{
    env, fs,
    path::PathBuf,
    process::{Child, Command},
};

use crate::{parser::BakaArgs, plugins::plugins, setting::project};

pub fn match_baka_flags(baka: BakaArgs) {
    match baka.baka_flags() {
        // Not found .baka.[json, toml, yaml]
        ("-p", Some(_)) => unimplemented!(),
        ("-l", Some(_)) => unimplemented!(),
        (_, _) => match_subcommand(baka),
    }
}

fn match_subcommand(baka: BakaArgs) {
    match baka.subcommand() {
        ("plugin", Some(plugin)) => plugin_commands(plugin),
        ("help", Some(_)) => println!("{}", include_str!("../../res/HELP")),
        ("version", Some(_)) => println!("{}", include_str!("../../res/VERSION")),

        // Found .baka.[json, toml, yaml]
        (_, _) => {
            unimplemented!("I found bug");
            /*
            if baka.subcommand.is_none() || baka.args.is_none() {
                return;
            }

            if let Some(project) = project() {
                let child = command_output(&project.manager,&baka.subcommand.as_ref().unwrap(), baka.args.as_ref().unwrap().to_vec());
                let wait_output = child.wait_with_output();

                if let Ok(output) = wait_output {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else if let Err(output) = wait_output {
                    eprintln!("Error: {}", output.to_string());
                }
            } else {
                eprintln!("Not found .baka.[json, toml, yaml]\nTry `baka [baka-flags] [package-manager-command] [package-manager-args]`")
            }
            */
        }
    }
}

fn plugin_commands(plugin: Vec<String>) {
    if plugin.is_empty() {
        return;
    }

    match plugin[0].as_str() {
        "add" => {
            if plugin.len() <= 1 || plugin.len() >= 3 {
                return;
            }

            let plugins_var = env::var("baka_plugins").unwrap();
            let mut path = PathBuf::from(plugins_var);
            if let Some(name) = plugin[1].split('/').last() {
                path.push(name.replace(".git", ""));
                let child = command_output(
                    "git",
                    "clone",
                    vec![plugin[1].clone(), path.to_string_lossy().to_string()],
                );
                let wait_output = child.wait_with_output();

                if let Ok(output) = wait_output {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!("Add {} plugin!", name.replace(".git", ""));
                } else if let Err(output) = wait_output {
                    eprintln!("Error: {}", output.to_string());
                }
            }
        }
        "remove" => {
            if plugin.len() <= 1 || plugin.len() >= 3 {
                return;
            }
            let mut find_plugin = plugins()
                .into_iter()
                .filter(|f| f.settings.name == plugin[1]);
            if let Some(plugin) = find_plugin.next() {
                let remove = fs::remove_dir_all(plugin.path);
                if remove.is_ok() {
                    println!("Remove {} plugin...", plugin.settings.name);
                } else if remove.is_err() {
                    println!("Error: {}", remove.unwrap_err());
                }
            }
        }
        "list" => {
            println!("Plugin list:");

            for plugin in plugins() {
                println!(
                    " ㄴname: {}   version: {}   path: {}",
                    plugin.settings.name,
                    plugin.settings.version,
                    plugin.path.to_string_lossy()
                );
            }
        }
        _ => {}
    }
}

fn command_output(program_name: &str, subcommand: &str, args: Vec<String>) -> Child {
    Command::new(program_name)
        .arg(subcommand)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("{} command failed to start", program_name))
}
