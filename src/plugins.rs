use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub cmd: BTreeMap<String, PluginCmd>,
    pub path: PluginPath,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PluginCmd {
    pub exec: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
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
