use std::env;
use std::process;
use oisuite::*;

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
            process::Command::new("git")
                .arg("clone")
        },
        "new" => {},
        "test" => {},
        _ => throw_lerror("Invalid argument!")
    };
}
