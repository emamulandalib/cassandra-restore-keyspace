mod restore_structure;
mod parse_commands;

use crate::restore_structure::{Restore};
// use crate::parse_commands::ParseCommand;
use cassandra_restore_structure::{Config};

fn main() {
    // let config = ParseCommand::new();
    let config = Config {
        cassandra_data_dir: "",
        tag: "",
        destination: "",
        key_space: "",
    };

    let restore = Restore::new(config);
    restore.run()
}
