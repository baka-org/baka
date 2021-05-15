use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    env::current_dir,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectSetting {
    pub manager: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RootSetting {
    pub language: String,
    pub plugins: Vec<String>,
    pub programming_languages: BTreeMap<String, String>,
}

pub fn root() -> RootSetting {
    let env = env::var("baka_root_setting").unwrap();
    let mut file = fs::File::open(env.clone()).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let root = {
        if env.ends_with("json") {
            serde_json::from_str::<RootSetting>(buf.as_str()).unwrap()
        } else if env.ends_with("toml") {
            toml::from_str::<RootSetting>(buf.as_str()).unwrap()
        } else if env.ends_with("yaml") {
            serde_yaml::from_str::<RootSetting>(buf.as_str()).unwrap()
        } else {
            panic!("Error: root setting file (serde)");
        }
    };

    root
}

pub fn project() -> ProjectSetting {
    let found_path = {
        let mut read_dir = fs::read_dir(current_dir().as_ref().unwrap()).unwrap();

        if read_dir.any(|x| x.unwrap().file_name() == ".baka.json") {
            ".baka.json"
        } else if read_dir.any(|x| x.unwrap().file_name() == ".baka.toml") {
            ".baka.toml"
        } else if read_dir.any(|x| x.unwrap().file_name() == ".baka.yaml") {
            ".baka.yaml"
        } else {
            panic!("Not Found: `.baka.[json, toml, yaml]`")
        }
    };

    let mut file = fs::File::open(found_path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let project = {
        if found_path.contains("json") {
            serde_json::from_str::<ProjectSetting>(buf.as_str()).unwrap()
        } else if found_path.contains("toml") {
            toml::from_str::<ProjectSetting>(buf.as_str()).unwrap()
        } else if found_path.contains("yaml") {
            serde_yaml::from_str::<ProjectSetting>(buf.as_str()).unwrap()
        } else {
            panic!("Error: project setting file (serde)");
        }
    };

    project
}

pub fn init() {
    //make .baka folder
    make_file(true, None, None, None);
    //make config file (global config file)
    let check_extension = vec!["config.json", "config.toml", "config.yaml"];
    let mut baka_folder = fs::read_dir(config_path(true, None, None).as_path()).unwrap();
    if !baka_folder.any(|x| check_extension.contains(&x.unwrap().file_name().to_str().unwrap())) {
        make_file(true, None, Some("config"), None);
    }
    //make plugins folder
    make_file(true, Some("plugins"), None, None);
    //set baka_root_setting
    if env::var("baka_root_setting").is_err() {
        let mut baka_folder2 = fs::read_dir(config_path(true, None, None).as_path()).unwrap();

        if baka_folder2.any(|x| x.unwrap().file_name().to_str() == Some("config.json")) {
            env::set_var(
                "baka_root_setting",
                config_path(true, None, Some("config.json")),
            );
        } else if baka_folder2.any(|x| x.unwrap().file_name().to_str() == Some("config.toml")) {
            env::set_var(
                "baka_root_setting",
                config_path(true, None, Some("config.toml")),
            );
        } else if baka_folder2.any(|x| x.unwrap().file_name().to_str() == Some("config.yaml")) {
            env::set_var(
                "baka_root_setting",
                config_path(true, None, Some("config.yaml")),
            );
        }
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

    if baka_folder {
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

pub fn make_file(
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
