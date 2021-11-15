use std::env;
use std::fs;
use std::process;
use std::process::Command;
use oisuite::throw_lerror;
use oisuite::print_help_text;
use std::io;

//#[warn(deprecated)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        throw_lerror("Not enough arguments provided!");
    }

    let option: &str = &args[1];
    match option {
        "help" => {
            print_help_text();
            process::exit(0);
        },
        "install" => {
            let path = env::home_dir().unwrap();
            let home: &str = path.to_str().unwrap();

            fs::create_dir_all(format!("{}/oi/projects", home));
            fs::create_dir_all(format!("{}/oi/.oisuite", home));

            Command::new("git")
                .arg("clone")
                .arg("https://www.github.com/querterdesu/oisuite-files")
                .arg(format!("{}/oi/.oisuite/project", home))
                .status()
                .expect("Failed to generate files");

        }
        "new" => {
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
        },
        "update" => {
            let path = env::home_dir().unwrap();
            let home: &str = path.to_str().unwrap();

            fs::remove_dir_all(format!("{}/oi/.oisuite/project", home));

            Command::new("git")
                .arg("clone")
                .arg("https://www.github.com/querterdesu/oisuite-files")
                .arg(format!("{}/oi/.oisuite/project", home))
                .status()
                .expect("Failed to generate files");
        },
        "test" => {},
        _ => throw_lerror("Invalid argument!")
    };
}
