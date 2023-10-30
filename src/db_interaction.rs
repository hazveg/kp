use keepass::{db::NodeRef, Database};

pub enum Action {
    List,
    Select,
}

#[derive(Debug)]
struct EntryData {
    title: String,
    password: String,
}

impl EntryData {
    fn new(title: &str, password: &str) -> Self {
        EntryData { title: title.to_string(), password: password.to_string() }
    }
}

pub fn define_action(action: &str) -> Action {
    match action {
        "list" => Action::List,
        "select" => Action::Select,
        _ => Action::List,
    }
}

#[allow(dead_code)]
pub fn debug_db_root(database: &Database) {
    println!("Entries:");
    for node in &database.root {
        match node {
            NodeRef::Entry(entry) => {
                println!("{}", entry.get_title().unwrap());
            }
            _ => {}
        }
    }
}

pub fn list_entries(database: &Database) {
    println!("Entries:");
    for node in &database.root {
        match node {
            NodeRef::Entry(entry) => {
                println!("{}", entry.get_title().unwrap());
            }
            _ => {}
        }
    }
}

pub fn select_entries(database: &Database, keyword: Option<String>) {
    if keyword.is_none() {
        eprintln!("no keywords given!");
    }

    let mut entries = vec![];
    for node in &database.root {
        match node {
            NodeRef::Entry(entry) => {
                let entry_title = entry.get_title().unwrap();
                
                // TODO: Make this fuzzy match
                if entry_title != keyword.as_ref().unwrap() {
                    continue;
                }

                entries.push(EntryData::new(
                    entry_title,
                    entry.get_password().unwrap(),
                ));
            },
            _ => {}
        }
    }

    dbg!(&entries);
}
