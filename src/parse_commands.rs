extern crate clap;

use std::fs::{read_to_string};
use std::path::Path;
use cassandra_restore_structure::{Config};
use clap::{Arg, App};

pub struct ParseCommand {}

impl ParseCommand {
    pub fn new() -> Result<Config, String> {
        let args = App::new("Cassandra KeySpace Restore")
            .version("0.0.1")
            .author("Emamul Andalib")
            .arg(Arg::with_name("config")
                .short("-c")
                .long("--config")
                .value_name("FILE")
                .takes_value(true)
                .validator(ParseCommand::is_valid_json_file)
                .help("Please specify a json file."))
            .get_matches();

        let filename = args.value_of("config").unwrap();
        let file_content = read_to_string(filename).unwrap();

        match serde_json::from_str(&file_content) {
            Ok(j) => Ok(j),
            Err(e) => Err(e.to_string())
        }
    }

    fn is_valid_json_file(v: String) -> Result<(), String> {
        let path = Path::new(&v);

        if !path.is_file() {
            return Err("Should be a file.".to_string());
        }

        match path.extension() {
            Some(e) => {
                if e == "json" {
                    return Ok(());
                }
                return Err(String::from("Should be a valid json file."));
            }
            None => Err(String::from("Should be a valid json file."))
        }
    }
}