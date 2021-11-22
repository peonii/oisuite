use std::env;
use std::fs;
use std::process;
use std::process::Command;
use oisuite::throw_lerror;
use oisuite::print_help_text;
use std::io;
use termion::color;
use termion::style;
use std::time::Instant;
extern crate yaml_rust;
use yaml_rust::{YamlLoader};

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
            let conf = "
            # Project config
            default_repo: https://www.github.com/querterdesu/oisuite-files 
            ";
            fs::write(format!("{}/oi/.oisuite/config.yml", home), conf).expect("Unable to write config file!");

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

            fs::create_dir_all(format!("{}/tests", name));
        },
        "update" => {
            let path = env::home_dir().unwrap();
            let home: &str = path.to_str().unwrap();
            let config_path = format!("{}/oi/.oisuite/config.yml", home);
            let config_str = fs::read_to_string(config_path).expect("Unable to read config file!");
            let config = YamlLoader::load_from_str(&config_str).unwrap();
            let default_repo = config[0]["default_repo"].as_str().unwrap();

            fs::remove_dir_all(format!("{}/oi/.oisuite/project", home));

            Command::new("git")
                .arg("clone")
                .arg(format!("{}", default_repo))
                .arg(format!("{}/oi/.oisuite/project", home))
                .status()
                .expect("Failed to generate files");
        },
        "generate" => {
            if args.len() <= 4 {
                throw_lerror("Not enough arguments provided!");
            }

            let packname = &args[2];
            let timelimit: u128 = args[3].parse::<u128>().unwrap();
            let limit: i32 = args[4].parse().unwrap();

            fs::remove_dir_all(format!("tests/{}", packname));
            fs::create_dir(format!("tests/{}", packname));

            println!("Compiling {}{}test generator{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);

            Command::new("g++")
                .arg("generate_tests.cpp")
                .arg("-o")
                .arg("gent")
                .status()
                .expect("Failed to generate testcase :(");

            println!("Compiling {}{}testing algorithm{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);
            Command::new("g++")
                .arg("brute.cpp")
                .arg("-o")
                .arg("brute")
                .status()
                .expect("Failed to generate testcase :(");

            let mut working: i32 = 0;
            fs::write(String::from(format!("tests/{}/testinfo", packname)), format!("0\n{}\n{}", timelimit, limit));

            for i in (1..limit+1) {

                let mut outp_i = Command::new("./gent");

                let mut temp_i = outp_i.output().expect("uwu");
                let mut stdout_i = String::from_utf8(temp_i.stdout).unwrap();

                fs::write(format!("tests/{}/{}.in", packname, i), stdout_i);

                let mut outp_o = Command::new("./brute");

                outp_o.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i)).unwrap());

                let mut temp_o = outp_o.output().expect("uwu2");
                let mut stdout_o = String::from_utf8(temp_o.stdout).unwrap();

                fs::write(format!("tests/{}/{}.out", packname, i), stdout_o);

                println!("{}🗸 Generated testcase {} successfully!{}", color::Fg(color::Green), i, color::Fg(color::Reset));
                working += 1;
            }
            fs::remove_file("gent");
            fs::remove_file("brute");
            println!("Successfully generated {}{}/{}{} testcases.", color::Fg(color::Green), working, limit, color::Fg(color::Reset));
        },
        "test" => {
            if args.len() <= 2 {
                throw_lerror("Not enough arguments provided!");
            }

            let packname = &args[2];
            println!("Compiling {}{}algorithm{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);

            Command::new("g++")
                .arg("main.cpp")
                .arg("-o")
                .arg("main")
                .status()
                .expect("Failed to compile :(");

            let testinfo = fs::read(format!("tests/{}/testinfo", packname)).unwrap();
            let detailedinfo = String::from_utf8(testinfo).unwrap();

            let mut split = detailedinfo.lines();
            let split_v: Vec<&str> = split.collect();

            let advanced_check = &split_v[0];
            let timelimit: u128 = split_v[1].parse::<u128>().unwrap();
            let amount: i32 = split_v[2].parse().unwrap();
            let ccc = "1";
            let mut passed = 0;

            if advanced_check == &ccc {
                let mut done = vec![false; amount as usize];

                let mut log = String::from("");
                for k in (2..amount+2) {
                    let i: usize = k as usize;
                    let req = split_v[i+1];
                    let mut can_exec = true;
                    if req != "n" {
                        let check: usize = req.parse::<i32>().unwrap() as usize;
                        if !done[check - 1] {
                            can_exec = false;
                        }
                    }
                    if can_exec {
                        let mut rmain = Command::new("./main");
                        rmain.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i-1)).unwrap());
                        let before = Instant::now();
                        let output = rmain.output().expect("Failed running algorithm!");
                        let after = Instant::now();
                        let stdout_output = String::from_utf8(output.stdout).unwrap();
                        let tdiff = after.duration_since(before).as_millis();

                        let expectedf = fs::read(format!("tests/{}/{}.out", packname, i-1)).unwrap();
                        let expected = String::from_utf8(expectedf).unwrap();


                        if stdout_output.trim() == expected.trim() {
                            if tdiff <= timelimit {
                                done[i-2] = true;
                                passed += 1;
                                println!("Testcase {} succeeded ({}ms)!", i-1, tdiff);
                            } else {
                                println!("Time for testcase {} exceeded! ({}/{}ms)", i-1, tdiff, timelimit);
                                log += format!("\n\nTestcase {}: Time limit exceeded ({}/{}ms)", i-1, tdiff, timelimit).as_str();
                            }
                        } else {
                            println!("Testcase {} failed ({}ms)!", i-1, tdiff);
                            log += format!("\n\nTestcase {}: Got \"{}\", expected \"{}\"", i-1, stdout_output, expected).as_str();
                        }
                    } else {
                        println!("Testcase {} skipped because testcase {} failed", i-1, req)
                    }
                }
                fs::write(format!("tests/{}/log", packname), log);
            } else {
                let mut log = String::from("");
                for k in (1..amount+1) {
                    let i: usize = k as usize;
                    let mut rmain = Command::new("./main");
                    rmain.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i)).unwrap());
                    let before = Instant::now();
                    let output = rmain.output().expect("Failed running algorithm!");
                    let after = Instant::now();
                    let stdout_output = String::from_utf8(output.stdout).unwrap();
                    let tdiff = after.duration_since(before).as_millis();

                    let expectedf = fs::read(format!("tests/{}/{}.out", packname, i)).unwrap();
                    let expected = String::from_utf8(expectedf).unwrap();


                    if stdout_output.trim() == expected.trim() {
                        if tdiff <= timelimit {
                            passed += 1;
                            println!("Testcase {} succeeded ({}ms)!", i, tdiff);
                        } else {
                            println!("Time for testcase {} exceeded! ({}/{}ms)", i, tdiff, timelimit);
                            log += format!("\n\nTestcase {}: Time limit exceeded ({}/{}ms)", i, tdiff, timelimit).as_str();
                        }
                    } else {
                        println!("Testcase {} failed ({}ms)!", i, tdiff);
                        log += format!("\n\nTestcase {}: Got \"{}\", expected \"{}\"", i, stdout_output, expected).as_str();
                    }
                }
                fs::write(format!("tests/{}/log", packname), log);
            }
            fs::remove_file("main");
            println!("Testing {} ended successfully. {}/{} testcases succeeded", packname, passed, amount);
        },
        _ => throw_lerror("Invalid argument!")
    };
}
