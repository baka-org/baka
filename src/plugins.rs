use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    fs::{self},
    io::Read,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PluginSetting {
    pub name: String,
    pub version: String,
    pub cmd: BTreeMap<String, PluginCmd>,
    pub path: PluginPath,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PluginCmd {
    pub exec: String,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PluginPath {
    // If all is null, should have at least one of darwin, win, Linux, other
    pub all: Option<String>,
    pub darwin: Option<String>,
    pub win: Option<String>,
    pub linux: Option<String>,
    pub other: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub path: PathBuf,
    pub settings: PluginSetting,
}

impl PluginSetting {
    // input: (plugin-cmd-key)
    // return: (plugin-cmd-exec, plugin-cmd-description, plugin-cmd-help)
    pub fn exec(&self, other: &str) -> Option<(String, String, String)> {
        self.cmd.get_key_value(other).map(|get| {
            (
                get.1.exec.replace("%path%", self.path()),
                get.1.help.as_ref().unwrap_or(&String::new()).to_string(),
                get.1.description.as_ref().unwrap_or(&String::new()).to_string(),
            )
        })
    }

    pub fn path(&self) -> &str {
        if self.path.all.is_none() {
            if cfg!(target_os = "linux") {
                self.path.linux
                    .as_ref()
                    .unwrap_or_else(|| panic!("Not found: `linux`"))
            } else if cfg!(target_os = "windows") {
                self.path.win
                    .as_ref()
                    .unwrap_or_else(|| panic!("Not found: `win`"))
            } else if cfg!(target_os = "darwin") {
                self.path.darwin
                    .as_ref()
                    .unwrap_or_else(|| panic!("Not found: `darwin`"))
            } else {
                self.path.other
                    .as_ref()
                    .unwrap_or_else(|| panic!("Not found: `other`"))
            }
        } else {
            self.path.all
                .as_ref()
                .unwrap_or_else(|| panic!("Not found: `all`"))
        }
    }
}

pub fn plugins() -> Vec<Plugin> {
    let mut plugins = Vec::new();

    let path = env::var("baka_plugins").unwrap();
    let read_dir = fs::read_dir(path).unwrap();
    let check_files = vec!["plugin.json", "plugin.toml", "plugin.yaml"];

    for dir in read_dir.flatten() {
        if dir.file_type().unwrap().is_file() {
            continue;
        }

        let dir_plugins = fs::read_dir(dir.path()).unwrap();

        for dir_plugin in dir_plugins.flatten() {
            let path = dir.path();

            if check_files.contains(&dir_plugin.file_name().to_str().unwrap()) {
                let mut file = fs::File::open(dir_plugin.path()).unwrap();
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();

                let serde = {
                    if dir_plugin.file_name() == "plugin.json" {
                        serde_json::from_str::<PluginSetting>(buf.as_str()).unwrap()
                    } else if dir_plugin.file_name() == "plugin.toml" {
                        toml::from_str::<PluginSetting>(buf.as_str()).unwrap()
                    } else if dir_plugin.file_name() == "plugin.yaml" {
                        serde_yaml::from_str::<PluginSetting>(buf.as_str()).unwrap()
                    } else {
                        panic!("Error: plugin setting file (serde)");
                    }
                };

                plugins.push(Plugin {
                    path,
                    settings: serde,
                })
            }
        }
    }

    plugins
}
