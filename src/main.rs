use keepass::{
    db::NodeRef,
    Database,
    DatabaseKey,
    error::DatabaseOpenError,
};
use std::fs::File;
use std::path::Path;
use clap::Parser;

#[derive(Default, Parser, Debug)]
#[clap(author="hazveg", version="0.1.0", about)]
/// A simple KeePass database reader
struct Arguments {
    /// The path to the database file
    path: String,
    /// The password to open the database
    password: String,
}

fn open_db_file(path: &Path, key: DatabaseKey) -> Result<Database, DatabaseOpenError> {
    let mut file = File::open(path)?;
    Database::open(&mut file, key)
}

#[allow(dead_code)]
fn debug_db_root(database: &Database) {
    for node in &database.root {
        match node {
            NodeRef::Group(group) => {
                println!("Group: {}", group.name);
            }
            NodeRef::Entry(entry) => {
                println!("Entry: {}", entry.get_title().unwrap());
            }
        }
    }
}

fn main() {
    let args = Arguments::parse();
    dbg!(&args);

    let path = std::path::Path::new("Database.kdbx");

    #[allow(unused_variables)]
    let database = match open_db_file(path, DatabaseKey::with_password(".DyLm69420.Hunter.651.")) {
        Ok(db) => db,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    //debug_db_root(&database);
}
