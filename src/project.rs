use std::env;
use std::fs;
use std::process;
use crate::*;
pub fn new(args: &Vec<String>) {
    let path = env::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();

    if args.len() <= 2 {
        throw_lerror("Not enough arguments provided!");
    }
    let name = &args[2];

    fs::create_dir_all(format!("{}", name));

    for file in fs::read_dir(format!("{}/oi/.oisuite/project", home)).unwrap() {
        fs::copy(file.as_ref().unwrap().path(), format!("{}/{}", name, file.unwrap().file_name().into_string().unwrap()));
    }

    fs::create_dir_all(format!("{}/tests", name));
}
