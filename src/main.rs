use std::env;

//#[warn(deprecated)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        oisuite::throw_lerror("Not enough arguments provided!");
    }

    let option: &str = &args[1];
    match option {
        "help" => oisuite::print_help_text(),
        "install" => oisuite::install::install(),
        "new" => oisuite::project::new(&args),
        "update" => oisuite::install::update(),
        "generate" => oisuite::testing::generate(&args),
        "test" => oisuite::testing::test(&args),
        "getsolution" => oisuite::fetcher::get_solution(&args),
        _ => oisuite::throw_lerror("Invalid argument!")
    };
}
