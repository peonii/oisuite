use std::fs;
use crate::*;
pub fn new(args: &Vec<String>) {
    let path = home::home_dir().unwrap();
    let home: &str = path.to_str().unwrap();

    if args.len() <= 2 {
        throw_lerror("Not enough arguments provided!");
    }
    let name = &args[2];

    match fs::create_dir_all(format!("{}", name)) {
        Ok(_) => {},
        Err(_) => {
            throw_error("Failed to create directory!");
        }
    };

    for file in fs::read_dir(format!("{}/oi/.oisuite/project", home)).unwrap() {
        match fs::copy(file.as_ref().unwrap().path(), format!("{}/{}", name, file.unwrap().file_name().into_string().unwrap())) {
            Ok(_) => {},
            Err(_) => {
                throw_error("Failed to copy default files!");
            }
        };
    }

    match fs::create_dir_all(format!("{}/tests", name)) {
        Ok(_) => {},
        Err(_) => {
            throw_error("Failed to create directory!");
        }
    };
}
