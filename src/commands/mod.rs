use crate::MY_DREAM;
use sha2::Sha512;
use std::{
    env,
    env::consts::OS,
    fs,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

use sha2::Digest;

use crate::{
    parser::BakaArgs,
    plugins::plugins,
    setting::{project, root},
};

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
        (command, args) => custon_command(command, &args),
    }
}

fn custon_command(command: &str, args: &Option<Vec<String>>) {
    let run_command = move |manager: String| {
        for plugin in plugins() {
            if plugin.settings.name != manager {
                println!("Couldn't find manager")
            } else {
                for (name, content) in plugin.settings.cmd.iter() {
                    if name == command {
                        let exec_command = content.exec.clone();
                        let plugin_path = plugin.settings.path.clone();
                        let path = match plugin_path.all {
                            None => match OS {
                                "macos" | "ios" => plugin_path.darwin,
                                "linux" | "android" => plugin_path.linux,
                                "windows" => plugin_path.win,
                                _ => plugin_path.other,
                            },
                            Some(ref value) => Some(value.to_string()),
                        };

                        let splited = exec_command.split(" ").map(|cmd| {
                            cmd.replace("%path%", path.as_ref().unwrap_or(&"".to_string()).as_str())
                        });

                        Command::new(splited[0])
                            .args(args.as_ref().unwrap_or(&Vec::new()))
                            .stdout(Stdio::inherit())
                            .stdin(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("Failed to run a command.");
                    }
                }
            }
        }
    };

    match project() {
        Some(proj) => {
            let manager = proj.manager;
            run_command(manager);
        }
        None => {
            // TODO: Make this to use arg-set manager
            println!("Project setting not found.")
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
                    println!("Added plugin {}!", name.replace(".git", ""));
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
                    println!("Removed plugin {}!", plugin.settings.name);
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
