use std::collections::HashMap;

const DB_FILENAME: &str = "kv.db";

fn main() {
    let database = Database::new().expect("Failed to create database!");

    let mut args = std::env::args().skip(1);
    let key = args.next().expect("TODO: SHOW USAGE");
    let value = args.collect::<Vec<String>>().join(" ");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let content = std::fs::read_to_string(DB_FILENAME)?;
        for line in content.lines() {
            let (key, value) = line.split_once('\t').expect("DATABASE CORRUPT");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map: map })
    }
}
