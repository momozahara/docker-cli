use dirs::home_dir;
use dotenv;
use std::{env, fs, io::Write};

pub fn load(profile: Option<&String>) -> (String, String, String) {
    let target_path = home_dir()
        .and_then(|a| {
            Some(a.join(format!(
                "pcode-cli/docker/{}.env",
                match profile {
                    Some(r) => r,
                    None => unreachable!(),
                }
            )))
        })
        .unwrap();

    env::remove_var("USERNAME");
    env::remove_var("HOSTNAME");
    env::remove_var("TARGET_PATH");

    dotenv::from_path(target_path.as_path()).expect("profile does not existed");

    let username = env::var("USERNAME").unwrap();
    let hostname = env::var("HOSTNAME").unwrap();
    let path = env::var("TARGET_PATH").unwrap();

    (username, hostname, path)
}

pub fn create(username: String, hostname: String, c_path: String, profile: Option<&String>) {
    let target_path = home_dir()
        .and_then(|a| {
            Some(a.join(format!(
                "pcode-cli/docker/{}.env",
                match profile {
                    Some(r) => r,
                    None => unreachable!(),
                }
            )))
        })
        .unwrap();
    let path = target_path.as_path();
    let prefix = path.parent().unwrap();

    fs::create_dir_all(prefix).unwrap();
    fs::remove_file(path).unwrap_or(());

    let mut file = fs::File::create(path).unwrap();
    file.write(
        format!(
            "USERNAME={}\nHOSTNAME={}\nTARGET_PATH={}",
            username, hostname, c_path
        )
        .as_bytes(),
    )
    .unwrap();
}
