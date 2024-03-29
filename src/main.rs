mod cli;
mod env;

use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    let matches = cli::init().get_matches();

    let profile = matches.get_one::<String>("profile");

    let username: String;
    let hostname: String;
    let path: String;
    let target: Option<&String>;
    let rmi: Option<&String>;

    match matches.subcommand() {
        Some(("env", sub_matches)) => {
            username = sub_matches.get_one::<String>("username").unwrap().clone();
            hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
            path = sub_matches.get_one::<String>("path").unwrap().clone();

            env::create(username, hostname, path, profile);
        }
        Some(("up", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                }
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                }
            }

            target = sub_matches.get_one::<String>("target");

            let output = Command::new("ssh")
                .arg("-tt")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {}", path))
                .arg("&& docker compose up")
                .arg("-d")
                .arg(match target {
                    Some(_target) => {
                        format!("--build {}", _target)
                    }
                    None => "".to_owned(),
                })
                .spawn()
                .expect("command failed to start");

            let output = output.wait_with_output().unwrap();

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Some(("down", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                }
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                }
            }

            rmi = sub_matches.get_one::<String>("rmi");

            let output = Command::new("ssh")
                .arg("-tt")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {}", path))
                .arg("&& docker compose down")
                .arg(match rmi {
                    Some(r) => format!("--rmi {}", r),
                    None => String::from(""),
                })
                .spawn()
                .expect("command failed to start");

            let output = output.wait_with_output().unwrap();

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Some(("start", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                }
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                }
            }

            target = sub_matches.get_one::<String>("target");

            let output = Command::new("ssh")
                .arg("-tt")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {} && docker compose start", path))
                .arg(match target {
                    Some(_target) => {
                        format!("{}", _target)
                    }
                    None => "".to_owned(),
                })
                .spawn()
                .expect("command failed to start");

            let output = output.wait_with_output().unwrap();

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Some(("stop", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                }
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                }
            }

            target = sub_matches.get_one::<String>("target");

            let output = Command::new("ssh")
                .arg("-tt")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {} && docker compose stop", path))
                .arg(match target {
                    Some(_target) => {
                        format!("{}", _target)
                    }
                    None => "".to_owned(),
                })
                .spawn()
                .expect("command failed to start");

            let output = output.wait_with_output().unwrap();

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        _ => unreachable!(),
    }
}
