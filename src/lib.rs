use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cassandra_data_dir: String,
    pub tag: String,
    pub key_space: String,
    pub destination: String,
}

pub fn show_err_msg(message: &str) {
    println!("{}", message);
    exit(1)
}