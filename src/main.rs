// TODO: implement function sync
// TODO: find a way to define "Project"
// TODO: exceptions handling
// TODO: configuration file
// FIX: date_format error

mod taskoto;
mod task;
mod database;
mod parser;
mod sync;

use serde_derive::{Serialize, Deserialize};
use taskoto::taskoto::taskoto_run;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{
    fs::{self, File},
    io::Write,
};

pub const CONFIG_DIR: &str = "/home/fs002905/.taskotorc";

pub const VALID_FORMAT_WITH_Y: [&str; 8] = [
    "%Y-%m-%d", "%m-%d-%Y", "%y-%m-%d", "%m-%d-%y",
    "%B %d, %Y", "%B %d, %y", "%b %d, %Y", "%b %d, %y",
]; 
pub const VALID_FORMAT_NO_Y: [&str; 3] = [
    "%m-%d", "%B %d", "%b %d",
]; 

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::init(CONFIG_DIR));
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    user_name: String,
    email: String,
    path: String,
    date_format: usize,
    sync: bool,
    sync_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_name: String::from("default"),
            email: String::from("default@default.com"),
            path: String::from("/home/fs002905/.taskoto/taskoto.db"),
            date_format: 1, 
            sync: false,
            sync_url: String::from("0.0.0.0"),
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
        let mut file = File::create(CONFIG_DIR).unwrap();
        file.write_all(toml_string.as_bytes()).unwrap();
    }

    fn use_default_config() -> Config {
        let config = Config::default();
        config.config_write();
        config
    }
}


/*CONFIG parameters*/
pub fn get_database_dir() -> String {
    CONFIG.lock().unwrap().path.clone()
}
pub fn get_date_format() -> usize {
    CONFIG.lock().unwrap().date_format
}
pub fn is_sync() -> bool {
    CONFIG.lock().unwrap().sync
}
pub fn get_sync_url() -> String {
    CONFIG.lock().unwrap().sync_url.clone()
}
pub fn get_user_name() -> String {
    CONFIG.lock().unwrap().user_name.clone()
}
pub fn get_email() -> String {
    CONFIG.lock().unwrap().email.clone()
}


fn main() {
    taskoto_run();
}
