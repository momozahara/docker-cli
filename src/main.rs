mod env;
mod cli;

use std::process::Command;

fn main() {
    let matches = cli::init().get_matches();

    let profile = matches.get_one::<String>("profile");

    let username: String;
    let hostname: String;
    let path: String;
    let rmi: Option<&String>;

    match matches.subcommand() {
        Some(("env", sub_matches)) => {
            username = sub_matches.get_one::<String>("username").unwrap().clone();
            hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
            path = sub_matches.get_one::<String>("path").unwrap().clone();

            env::create(username, hostname, path, profile);
        },
        Some(("up", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                },
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                },
            }

            let output = Command::new("ssh")
                .arg("-t")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {}", path))
                .arg("&& docker compose up")
                .arg("-d")
                .output()
                .expect("command failed to start");
            let stdout = output.stdout.as_slice();
            println!("{}", std::str::from_utf8(stdout).unwrap());
        },
        Some(("down", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                },
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                },
            }
            rmi = sub_matches.get_one::<String>("rmi");

            let output = Command::new("ssh")
                .arg("-t")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {}", path))
                .arg("&& docker compose down")
                .arg(
                    match rmi {
                        Some(r) => format!("--rmi {}", r),
                        None => String::from(""),
                    }
                )
                .output()
                .expect("command failed to start");
            let stdout = output.stdout.as_slice();
            println!("{}", std::str::from_utf8(stdout).unwrap());
        },
        Some(("start", sub_matches)) => {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                },
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                },
            }

            let output = Command::new("ssh")
                .arg("-t")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {} && docker compose start", path))
                .output()
                .expect("command failed to start");
            let stdout = output.stdout.as_slice();
            println!("{}", std::str::from_utf8(stdout).unwrap());
        },
        Some(("stop", sub_matches))=> {
            match profile {
                Some(_) => {
                    (username, hostname, path) = env::load(profile);
                },
                None => {
                    username = sub_matches.get_one::<String>("username").unwrap().clone();
                    hostname = sub_matches.get_one::<String>("hostname").unwrap().clone();
                    path = sub_matches.get_one::<String>("path").unwrap().clone();
                },
            }

            let output = Command::new("ssh")
                .arg("-t")
                .arg(format!("{}@{}", username, hostname))
                .arg(format!("cd {} && docker compose stop", path))
                .output()
                .expect("command failed to start");
            let stdout = output.stdout.as_slice();
            println!("{}", std::str::from_utf8(stdout).unwrap());
        },
        _ => unreachable!(),
    }
}
