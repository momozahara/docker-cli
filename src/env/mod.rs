use dotenv;
use dirs::home_dir;
use std::{
  env,
  fs, io::Write,
};

pub fn load(profile: Option<&String>) -> (String, String, String) {
  let target_path = home_dir().and_then(|a| Some(
    a.join(
      format!(
        "pcode-cli/docker/{}.env",
        match profile {
          Some(r) => r,
          None => unreachable!()
        }
      )
    )
  )).unwrap();

  env::remove_var("username");
  env::remove_var("hostname");
  env::remove_var("target_path");

  dotenv::from_path(target_path.as_path()).unwrap();

  let username = env::var("username").unwrap();
  let hostname = env::var("hostname").unwrap();
  let path = env::var("target_path").unwrap();

  (username, hostname, path)
}

pub fn create(username: String, hostname: String, c_path: String, profile: Option<&String>) {
  let target_path = home_dir().and_then(|a| Some(
    a.join(
      format!(
        "pcode-cli/docker/{}.env",
        match profile {
          Some(r) => r,
          None => unreachable!()
        }
      )
    )
  )).unwrap();
  let path = target_path.as_path();
  let prefix = path.parent().unwrap();

  fs::create_dir_all(prefix).unwrap();
  fs::remove_file(path).unwrap_or(());

  let mut file = fs::File::create(path).unwrap();
  file.write(format!("USERNAME={}\nHOSTNAME={}\nTARGET_PATH={}", username, hostname, c_path).as_bytes()).unwrap();
}