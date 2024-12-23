use serde::{Serialize, Deserialize};
use toml;
use toml::de::Error;

const BASE_RANGE: i32 = 1024;
const BASE_DEFAULT_POSITION: i32 = 0;
// WARN: can crash program

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub sensitivity: i32,
  pub dead_zone: i32,
  pub flat: i32,
  pub gui: bool,
}





impl Config {
  pub fn default() -> Self {
    Self {
      sensitivity: 1,
      dead_zone: 0,
      flat: 0,
      gui: true,
    }
  }

  pub fn exists() -> bool {
    match dirs::config_dir() {
      Some(_) => {
        return std::path::Path::new(&Config::path()).exists()
      }
      None => {
        return false
      }
    }
  }

  pub fn path() -> String {

    // Get the SUDO_USER environment variable to find the actual user
    let user = std::env::var("SUDO_USER").unwrap_or_else(|_| String::from("root"));
    
    // If running as sudo, construct path using the real user's home
    if user != "root" {
        format!("/home/{}/.config/mouse2joy/config.toml", user)
    } else {
        // Fallback to root's config if not running through sudo
        dirs::config_dir()
            .unwrap_or_default()
            .join("mouse2joy")
            .join("config.toml")
            .to_string_lossy()
            .into_owned()
    }
  }

  // WARN: Can make program crash
  pub fn load() -> Result<Config,Error> {
    let file = Config::path();
    let contents = std::fs::read_to_string(file).unwrap();
    toml::from_str(&contents)
  }

  pub fn range_min(&self) -> i32 {
    -(self.sensitivity*BASE_RANGE)/2
  }

  pub fn range_max(&self) -> i32 {
    (self.sensitivity*BASE_RANGE)/2
  }

  pub fn fuzz(&self) -> i32 {
    self.dead_zone
  }

  pub fn value(&self) -> i32 {
    BASE_DEFAULT_POSITION
  }

  pub fn resolution(&self) -> i32 {
    BASE_RANGE
  }

  pub fn flat(&self) -> i32 {
    self.flat
  }
}
