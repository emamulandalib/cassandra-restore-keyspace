mod restore_structure;
mod parse_commands;

use crate::restore_structure::{Restore};
use crate::parse_commands::ParseCommand;
use cassandra_restore_keyspace::show_err_msg;

fn main() {
    match ParseCommand::new() {
        Ok(config) => {
            let restore = Restore::new(config);
            restore.run()
        }
        Err(e) => show_err_msg(&e.to_string())
    }
}
