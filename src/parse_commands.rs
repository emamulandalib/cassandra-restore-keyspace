use std::{env};
use cassandra_restore_structure::Config;

pub struct ParseCommand {}

impl ParseCommand {
    pub fn new() -> Config {
        let mut data_dir = String::new();
        let mut tag = String::new();
        let mut key_space = String::new();
        let mut destination = String::new();

        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);

        for (i, arg) in args.iter().enumerate() {
            // let arg = arg.parse().unwrap();

            if *arg == String::from("--data-dir") {
                data_dir = String::from(arg)
            }
        }

        Config {
            cassandra_data_dir: "",
            tag: "",
            key_space: "",
            destination: "",
        }
    }
}