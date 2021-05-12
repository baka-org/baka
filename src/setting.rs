use std::{collections::BTreeMap, path::PathBuf};
use std::{env, fs, io::Write, path::Path};

use serde::{Deserialize, Serialize};

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
    let r: RootSetting = serde_json::from_str(include_str!("../settings-files/baka.json")).unwrap();

    println!("{:?}", r);
}

pub fn read_project_setting() {
    let r: ProjectSetting =
        serde_json::from_str(include_str!("../settings-files/bakaconfig.json")).unwrap();

    println!("{:?}", r);
}

pub fn init_config() {
    //make .baka folder
    make_config(true, None, None, None);
    //make config file (global config file)
    make_config(true, None, Some("config"), None);
    //make plugins folder
    make_config(true, Some("plugins"), None, None);
    //set baka_root_setting
    if env::var("baka_root_setting").is_err() {
        env::set_var("baka_root_setting", config_path(true, None, Some("config")));
    }
    //set baka_plugins
    if env::var("baka_plugins").is_err() {
        env::set_var("baka_plugins", config_path(true, Some("plugins"), None));
    }
}

fn config_path(baka_folder: bool, folder_name: Option<&str>, file_name: Option<&str>) -> PathBuf {
    let mut var = String::new();

    if cfg!(target_os = "windows") {
        var = std::env::var("USERPROFILE").unwrap();
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        var = std::env::var("HOME").unwrap();
    }

    let mut config_path = PathBuf::new();

    config_path.push(var);

    if baka_folder == true {
        config_path.push(".baka");
    }

    if let Some(folder) = folder_name {
        config_path.push(folder);
    }

    if let Some(file) = file_name {
        config_path.push(file);
    }

    config_path
}

pub fn make_config(
    baka_folder: bool,
    folder_name: Option<&str>,
    file_name: Option<&str>,
    text: Option<&str>,
) {
    let path_folder = config_path(baka_folder, folder_name, None);
    let path_file = config_path(baka_folder, folder_name, file_name);

    if !Path::new(&path_folder).exists() {
        fs::create_dir(&path_folder).unwrap();
    }

    if !Path::new(&path_file).exists() {
        let mut file = fs::File::create(&path_file).unwrap();
        if let Some(text) = text {
            file.write_all(text.as_bytes()).unwrap();
        }
        fs::metadata(&path_file)
            .unwrap()
            .permissions()
            .set_readonly(true);
    }
}
