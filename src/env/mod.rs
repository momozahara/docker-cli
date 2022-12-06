use dotenv;
use dirs::home_dir;
use std::{
  env,
  fs, io::Write,
};

pub fn init() -> (&'static str, &'static str, &'static str) {
  let home_path = home_dir().and_then(|a| Some(a.join("pcode-cli/docker/default.env"))).unwrap();
  dotenv::from_path(home_path.as_path()).unwrap_or(());

  let username = Box::leak(env::var("d_username").unwrap_or(String::from("")).into_boxed_str());
  let hostname = Box::leak(env::var("d_hostname").unwrap_or(String::from("")).into_boxed_str());
  let path = Box::leak(env::var("d_path").unwrap_or(String::from("")).into_boxed_str());

  (username, hostname, path)
}

pub fn create(username: &String, hostname: &String, d_path: &String) {
  let home_path = home_dir().and_then(|a| Some(a.join("pcode-cli/docker/default.env"))).unwrap();
  let path = home_path.as_path();
  let prefix = path.parent().unwrap();
  fs::create_dir_all(prefix).unwrap();
  fs::remove_file(path).unwrap_or(());
  let mut file = fs::File::create(path).unwrap();
  file.write(format!("D_USERNAME={}\nD_HOSTNAME={}\nD_PATH={}", username, hostname, d_path).as_bytes()).unwrap();
}