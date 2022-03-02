use std::fs;
use std::process;
use crate::*;
extern crate yaml_rust;
use yaml_rust::{YamlLoader};
pub fn install() {
    let path = home::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();

    match fs::create_dir_all(format!("{}/oi/projects", home)) {
        Ok(_) => {},
        Err(_) => throw_error("Could not create directory!"),
    };

    match fs::create_dir_all(format!("{}/oi/.oisuite", home)) {
        Ok(_) => {},
        Err(_) => throw_error("Could not create directory!"),
    };

    let conf = "##############################
# OISUITE CONFIGURATION FILE #
##############################

# Default project configuration
default_repo: https://www.github.com/querterdesu/oisuite-files
solutions_repo: https://www.github.com/querterdesu/public-algo-question-repository


# Made by Querter-chan#6666";
    fs::write(format!("{}/oi/.oisuite/config.yml", home), conf).expect("Unable to write config file!");

    clone_repo("https://www.github.com/querterdesu/oisuite-files");
}

pub fn update() {
    let path = home::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();
    let config_path = format!("{}/oi/.oisuite/config.yml", home);
    let config_str = fs::read_to_string(config_path).expect("Unable to read config file!");
    let config = YamlLoader::load_from_str(&config_str).unwrap();
    let default_repo = config[0]["default_repo"].as_str().unwrap();

    match fs::remove_dir_all(format!("{}/oi/.oisuite/project", home)) {
        Ok(_) => {},
        Err(_) => throw_error("Could not remove directory!"),
    };

    clone_repo(default_repo);
}

fn clone_repo(repo: &str) {
    let path = home::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();

    process::Command::new("git")
        .arg("clone")
        .arg(format!("{}", repo))
        .arg(format!("{}/oi/.oisuite/project", home))
        .status()
        .expect("Failed to generate files");
}