use std::fs::{create_dir_all, copy, read_dir};
use walkdir::{WalkDir, DirEntry};
use regex::Regex;
use cassandra_restore_keyspace::{Config, show_err_msg};

pub struct Restore {
    config: Config
}

impl Restore {
    pub fn new(config: Config) -> Restore {
        Restore { config }
    }

    fn is_root_dir_exists(&self, root: &str) -> bool {
        match read_dir(root) {
            Ok(_) => true,
            _ => false
        }
    }

    pub fn run(&self) {
        let root = format!(
            "{}/{}/", self.config.cassandra_data_dir, self.config.key_space
        );

        if !self.is_root_dir_exists(&root) {
            show_err_msg(&format!("Invalid directory '{}' for keyspace.", &root))
        }

        let walker = WalkDir::new(&root).into_iter();

        for entry in walker
            .filter_map(Result::ok)
            .filter(|e| self.is_tag_dir(e)) {
            let table_name = match entry.path().to_str() {
                Some(t) => {
                    self.get_table_name_from_path(&root, &t)
                }
                None => String::new()
            };

            if table_name.is_empty() {
                show_err_msg(("Invalid table name - '{}'".to_string() + table_name.as_ref()).as_ref())
            }

            let mut destination_dir = String::from(&self.config.destination);
            let is_destination_ends_with_slash = destination_dir.ends_with("/");

            if !is_destination_ends_with_slash {
                destination_dir.push_str("/");
            }

            let destination_dir = format!(
                "{}{}/{}", destination_dir, self.config.key_space, table_name
            );
            create_dir_all(&destination_dir).unwrap();

            let files = read_dir(entry.path()).unwrap();

            for file in files {
                let file = file.unwrap();
                let file_type = file.file_type().unwrap();
                let file_path = file.path();
                let filename = file.file_name();

                if file_type.is_file() {
                    let source_file = file_path.to_str().unwrap();
                    let source_file = format!("{}", source_file);
                    let filename = filename.to_str().unwrap();
                    let destination_file = format!("{}/{}", destination_dir, filename);
                    copy(&source_file, &destination_file).unwrap();
                }
            }
        }

        println!("Your files copied successfully into '{}'", self.config.destination)
    }

    fn is_tag_dir(&self, entry: &DirEntry) -> bool {
        entry.path().ends_with(&self.config.tag)
    }

    fn get_table_name_from_path(&self, root_path: &str, path: &str) -> String {
        let snapshot_with_tag = format!("{}/{}", "snapshots", self.config.tag);
        let t = path.replace(&root_path, "");
        let t = t.replace(&snapshot_with_tag, "");
        let re = Regex::new("-.*").unwrap();
        let t = re.replace_all(&t, "");
        format!("{}", t)
    }
}