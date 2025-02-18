// TODO: find a way to define "Project"
// TODO: exceptions handling
// TODO: 

mod taskoto;
mod task;
mod database;
mod parser;

use serde_derive::{Serialize, Deserialize};
use home;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{
    fs::{self, File},
    io::Write,
};

// pub const CONFIG_DIR: &str = "/home/fs002905/.taskotorc";
pub const CONFIG_NAME: &str = "/.taskotorc";

pub const VALID_FORMAT_WITH_Y: [&str; 8] = [
    "%Y-%m-%d", "%m-%d-%Y", "%y-%m-%d", "%m-%d-%y",
    "%B %d, %Y", "%B %d, %y", "%b %d, %Y", "%b %d, %y",
]; 
pub const VALID_FORMAT_NO_Y: [&str; 3] = [
    "%m-%d", "%B %d", "%b %d",
]; 

lazy_static! {
    // pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::init(CONFIG_DIR));
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::init(
        &get_config_dir()
    ));
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: String,
    date_format: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: String::from(&get_config_dir()),
            date_format: 1, 
        }
    }
}

impl Config {
    fn init(dir: &str) -> Self {
        match fs::read_to_string(dir) {
            Ok(toml_string) => match toml::from_str(&toml_string) {
                Ok(config) => config,
                Err(_) => Self::use_default_config(),
            },
            Err(_) => Self::use_default_config(), 
        }
    }
    fn config_write(&self) {
        let toml_string = toml::to_string(&self).unwrap();
        let mut file = File::create(&get_config_dir()).unwrap();
        file.write_all(toml_string.as_bytes()).unwrap();
    }

    fn use_default_config() -> Config {
        let config = Config::default();
        config.config_write();
        config
    }
}


// CONFIG parameters
pub fn get_config_dir() -> String {
    String::from(home::home_dir().unwrap().to_string_lossy()) + CONFIG_NAME
}
pub fn get_database_dir() -> String {
    CONFIG.lock().unwrap().path.clone()
}
pub fn get_date_format() -> usize {
    CONFIG.lock().unwrap().date_format
}

// Meaningless function only used for git test
pub fn what_the_fuck() -> String {
    String::from("What the fuck!")
}


fn main() {
    taskoto::taskoto::taskoto_run();
}
