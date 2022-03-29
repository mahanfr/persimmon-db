use std::{
  fs,
  io::{Read, Write},
  path::Path,
};

#[derive(Clone)]
pub(crate) struct Config {
  memory_buffer_size: usize,
  page_size: usize,
  deallocation_policy: String,
  system_cashing: bool,
  db_path: String,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      memory_buffer_size: 4096,
      page_size: 4096,
      deallocation_policy: "LRU".to_string(),
      system_cashing: true,
      db_path: "./data".to_string(),
    }
  }
}

impl Config {
  pub fn create() -> Config {
      if Path::new("./data/service.conf").exists() {
        println!("reading config from file");
        return load_config();
      }

      let mut config = String::new();
      config.push_str("# memory_buffer_size in MB\n");
      config.push_str("memory_buffer_size=128\n");
      config.push_str("# page_size in bytes\n");
      config.push_str("page_size=4096\n");
      config.push_str("# deallocation_policy: \"first_fit\" or \"best_fit\"\n");
      config.push_str("deallocation_policy=\"LRU\"\n");
      config.push_str("# system cashing: true or false\n");
      config.push_str("# disable for better memory management\n");
      config.push_str("system_cashing=true\n");
      config.push_str("# database_path: path to the database\n");
      config.push_str("database_path=\"./data\"\n");

      let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./data/service.conf")
        .unwrap();

      file.write_all(config.as_bytes()).unwrap();
      return Config::default();
  }

}
fn load_config() -> Config {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("./data/service.conf")
        .unwrap();
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {
            return parse_config(buffer);
        }
        Err(e) => {
            println!("ERROR: {}", e);
            return Config::default();
        }
    }
}

fn parse_config(content: String) -> Config{
  let mut config = Config::default();
  for line in content.lines() {
    if !line.starts_with("#") {
      let mut parts = line.split("=");
      let key = parts.next().unwrap().trim();
      let value = parts.next().unwrap().trim();
      match key {
        "memory_buffer_size" => {
            let size = value.parse::<usize>().unwrap();
            config.memory_buffer_size = size;
        }
        "page_size" => {
            let size = value.parse::<usize>().unwrap();
            config.page_size = size;
        }
        "deallocation_policy" => {
            let policy = value.parse::<String>().unwrap();
            config.deallocation_policy = policy;
        }
        "system_cashing" => {
            let caching = value.parse::<bool>().unwrap();
            config.system_cashing = caching;
        }
        "database_path" => {
            let path = value.parse::<String>().unwrap();
            config.db_path = path;
        }
        _ => {
            println!("ERROR: reading config file unknown key {}", key);
        }
      }
    }
  }
  return config;
}
