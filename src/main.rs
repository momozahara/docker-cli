mod env;
mod cli;

use std::process::Command;

fn main() {
    let matches = cli::init(env::init()).get_matches();

    match matches.subcommand() {
        Some(("env", sub_matches)) => {
            let username = sub_matches.get_one::<String>("username").unwrap();
            let hostname = sub_matches.get_one::<String>("hostname").unwrap();
            let path = sub_matches.get_one::<String>("path").unwrap();

            env::create(username, hostname, path);
        },
        Some(("start", sub_matches)) => {
            let username = sub_matches.get_one::<String>("username").unwrap();
            let hostname = sub_matches.get_one::<String>("hostname").unwrap();
            let path = sub_matches.get_one::<String>("path").unwrap();

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
            let username = sub_matches.get_one::<String>("username").unwrap();
            let hostname = sub_matches.get_one::<String>("hostname").unwrap();
            let path = sub_matches.get_one::<String>("path").unwrap();

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
