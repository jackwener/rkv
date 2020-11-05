use std::collections::HashMap;

use std::path::PathBuf;

enum Command {
    Set {key: String, value: String},
    Remove {key: String},
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

pub struct KvStore {
    path: PathBuf,
    mem_table: HashMap<String, usize>,
    cur: usize,
    threshold: usize,
}