use std::{
    path::Path,
    fs::{read_to_string},
};
use clap::{Arg, App};
use cassandra_restore_keyspace::Config;


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
                .required(true)
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
        let err_msg = String::from("Should be a valid json file.");

        if !path.is_file() {
            return Err(err_msg);
        }

        match path.extension() {
            Some(e) => {
                if e == "json" {
                    return Ok(());
                }
                return Err(err_msg);
            }
            None => Err(err_msg)
        }
    }
}