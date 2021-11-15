use std::env;
use std::process;
use std::process::Command;
use oisuite::throw_lerror;
use oisuite::print_help_text;
use std::io;

#[warn(deprecated)]
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

            Command::new("mkdir")
                .arg(format!("{}/oi", home))
                .spawn()
                .expect("Failed to install");

            Command::new("mkdir")
                .arg(format!("{}/oi/.oisuite/", home))
                .spawn()
                .expect("Failed to install");

            Command::new("mkdir")
                .arg(format!("{}/oi/projects/", home))
                .spawn()
                .expect("Failed to install");

            Command::new("git")
                .arg("clone")
                .arg("https://www.github.com/querterdesu/oisuite-files")
                .arg(format!("{}/oi/.oisuite/project", home))
                .spawn()
                .expect("Failed to generate files");

        }
        "new" => {
            let path = env::home_dir().unwrap();
            let home: &str = path.to_str().unwrap();

            if args.len() <= 2 {
                throw_lerror("Not enough arguments provided!");
            }
            let name = &args[2];

            Command::new("cp")
                .arg("-r")
                .arg(format!("{}/oi/.oisuite/project", home))
                .arg(format!("{}", name))
                .spawn()
                .expect("Unable to gen file");
        },
        "run" => {
            Command::new("g++")
                .arg("main.cpp")
                .arg("-o")
                .arg("main")
                .spawn()
                .expect("Unable to compile");

            Command::new("./main")
                .spawn()
                .expect("Unable to run file");
        },
        "build" => {
            Command::new("g++")
                .arg("main.cpp")
                .arg("-o")
                .arg("main")
                .spawn()
                .expect("Unable to compile");
        },
        "test" => {},
        _ => throw_lerror("Invalid argument!")
    };
}
