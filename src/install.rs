use std::env;
use std::fs;
use std::process;
extern crate yaml_rust;
use yaml_rust::{YamlLoader};
pub fn install() {
    let path = env::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();

    fs::create_dir_all(format!("{}/oi/projects", home));
    fs::create_dir_all(format!("{}/oi/.oisuite", home));
    let conf = "
    # Project config
    default_repo: https://www.github.com/querterdesu/oisuite-files 
    ";
    fs::write(format!("{}/oi/.oisuite/config.yml", home), conf).expect("Unable to write config file!");

    process::Command::new("git")
        .arg("clone")
        .arg("https://www.github.com/querterdesu/oisuite-files")
        .arg(format!("{}/oi/.oisuite/project", home))
        .status()
        .expect("Failed to generate files");
}

pub fn update() {
    let path = env::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();
    let config_path = format!("{}/oi/.oisuite/config.yml", home);
    let config_str = fs::read_to_string(config_path).expect("Unable to read config file!");
    let config = YamlLoader::load_from_str(&config_str).unwrap();
    let default_repo = config[0]["default_repo"].as_str().unwrap();

    fs::remove_dir_all(format!("{}/oi/.oisuite/project", home));

    process::Command::new("git")
        .arg("clone")
        .arg(format!("{}", default_repo))
        .arg(format!("{}/oi/.oisuite/project", home))
        .status()
        .expect("Failed to generate files");
}