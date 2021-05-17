use crate::plugins::plugin_find;
use crate::MY_DREAM;
use sha2::Sha512;
use std::{
    env, fs,
    path::PathBuf,
    process::{Child, Command},
};

use sha2::Digest;

use crate::{parser::BakaArgs, plugins::plugins, setting::root};

pub fn match_baka_flags(baka: BakaArgs) {
    match baka.baka_flags() {
        // Not found .baka.[json, toml, yaml]
        ("-p", Some(_)) => {
            unimplemented!();
            //let plugin = plugins();
            //println!("{:?}", plugin[0].settings.exec("search"))
        }
        ("-l", Some(_)) => {
            //TODO: check plugin
            if baka.subcommand.is_none() {
                return;
            }

            if let Some(lang) = root()
                .programming_languages
                .get(&baka.baka_flags.as_ref().unwrap()[1])
            {
                let child =
                    command_output(lang.as_str(), &baka.subcommand.as_ref().unwrap(), baka.args);
                let wait_output = child.wait_with_output();

                if let Ok(output) = wait_output {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else if let Err(output) = wait_output {
                    eprintln!("Error: {}", output.to_string());
                }
            }
        }
        (_, _) => match_subcommand(baka),
    }
}

fn match_subcommand(baka: BakaArgs) {
    match baka.subcommand() {
        ("plugin", Some(plugin)) => plugin_commands(plugin),
        ("help", Some(_)) => println!("{}", include_str!("../../res/HELP")),
        ("version", Some(_)) => println!("{}", include_str!("../../res/VERSION")),

        // Found .baka.[json, toml, yaml]
        (_, _) => unimplemented!("I found bug"),
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
                let mut hasher = Sha512::new();
                hasher.update(format!("{}-{}", MY_DREAM, name.replace(".git", "")).as_bytes());
                path.push(
                    hasher
                        .finalize()
                        .to_vec()
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<String>(),
                );

                let child = command_output(
                    "git",
                    "clone",
                    Some(vec![plugin[1].clone(), path.to_string_lossy().to_string()]),
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

fn command_output(program_name: &str, subcommand: &str, args: Option<Vec<String>>) -> Child {
    Command::new(program_name)
        .arg(subcommand)
        .args(args.unwrap_or_default())
        .spawn()
        .unwrap_or_else(|_| panic!("{} command failed to start", program_name))
}
