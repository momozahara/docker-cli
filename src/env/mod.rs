use dotenv;
use dirs::home_dir;
use std::{
  env,
  fs, io::Write,
};

pub fn init() -> (&'static str, &'static str, &'static str) {
  let home_path = home_dir().and_then(|a| Some(a.join("pcode-cli/docker/.env"))).unwrap();
  let results = dotenv::from_path(home_path.as_path());

  let mut username = "";
  let mut hostname = "";
  let mut path = "";

  match results {
    Ok(_) => {
      username = Box::leak(env::var("username").unwrap().into_boxed_str());
      hostname = Box::leak(env::var("hostname").unwrap().into_boxed_str());
      path = Box::leak(env::var("docker_path").unwrap().into_boxed_str());
    },
    Err(_) => (),
  }

  (username, hostname, path)
}

pub fn create(username: &String, hostname: &String, d_path: &String) {
  let home_path = home_dir().and_then(|a| Some(a.join("pcode-cli/docker/.env"))).unwrap();
  let path = home_path.as_path();
  let prefix = path.parent().unwrap();
  fs::create_dir_all(prefix).unwrap();
  fs::remove_file(path).unwrap_or(());
  let mut file = fs::File::create(path).unwrap();
  file.write(format!("USERNAME={}\nHOSTNAME={}\nDOCKER_PATH={}", username, hostname, d_path).as_bytes()).unwrap();
}