use std::process;

pub fn print_help_text() {
    println!("LIST OF COMMANDS");
    println!("\thelp - shows this list");
    println!("\tinstall - installs oisuite onto your machine");
    println!("\tnew <project name> - makes a new C++ project with the specified name");
    println!("\ttest - tests the current C++ project");
}

pub fn throw_lerror(message: &str) {
    println!("{}", message);
    println!("Type \"oisuite help\" for a list of commands");

    process::exit(1);
}
