use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub cmd: PluginCmd,
    pub path: PluginPath,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginCmd {
    pub install: String,
    pub uninstall: String,
    pub search: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginPath {
    // If all is null, should have at least one of darwin, win, Linux, other
    pub all: Option<String>,
    pub darwin: Option<String>,
    pub win: Option<String>,
    pub linux: Option<String>,
    pub other: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectSetting {
    pub manager: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RootSetting {
    pub language: String,
    pub plugins: Vec<String>,
    pub programming_languages: BTreeMap<String, String>,
}

pub fn read_root_setting() {
    let r: RootSetting =
        serde_json::from_str(include_str!("../settings-files/.baka.json")).unwrap();

    println!("{:?}", r);
}

pub fn read_project_setting() {
    let r: ProjectSetting =
        serde_json::from_str(include_str!("../settings-files/.bakaconfig.json")).unwrap();

    println!("{:?}", r);
}

pub fn read_plugin() {
    let r: Plugin = serde_json::from_str(include_str!("../plugins-files/plugin.json")).unwrap();

    println!("{:?}", r);
}
