pub mod install;
pub mod project;
pub mod testing;

use std::process;
pub use crate::install::*;
pub use crate::project::*;
pub use crate::testing::*;

pub fn print_help_text() {
    println!("LIST OF COMMANDS");
    println!("\thelp - shows this list");
    println!("\tinstall - fetches all the required templates for oisuite");
    println!("\tupdate - updates all the required templates for oisuite (WILL REPLACE YOUR EXISTING TEMPLATES)");
    println!("\tnew <project name> - makes a new C++ project with the specified name");
    println!("\ttest - tests the current C++ project");
}

pub fn throw_lerror(message: &str) {
    println!("{}", message);
    println!("Type \"oisuite help\" for a list of commands");

    process::exit(1);
}

pub fn throw_error(message: &str) {
    println!("ERROR: {}", message);
    process::exit(1);
}