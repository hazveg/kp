use copypasta::{ClipboardProvider, ClipboardContext};
use keepass::{db::NodeRef, Database};

use crate::string_match;

pub enum Action {
    List,
    Select,
}

#[derive(Debug)]
pub struct EntryData {
    title: String,
    username: String,
    password: String,
}

impl EntryData {
    fn new(title: &str, username: &str,  password: &str) -> Self {
        EntryData {
            title: title.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    fn output(&self, clipboard_context: &mut ClipboardContext) {
        let success;
        match clipboard_context.set_contents(self.password.to_owned()) {
            Ok(_) => success = true,
            Err(_) => success = false,
        }

        println!("------- {} --------", self.title);
        println!("Username: {}", self.username);
        if success {
            println!("Password: [copied to clipboard]");
        } else {
            eprintln!("Clipboard Insertion failed, displaying instead");
            println!("Password: {}", self.password);
        }
        println!();
    }

    fn title(&self) -> &str {
        &self.title[..]
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
    for node in &database.root {
        match node {
            NodeRef::Entry(entry) => {
                println!("{}", entry.get_title().unwrap());
            }
            _ => {}
        }
    }
}

pub fn select_entries(database: &Database, keyword: &Option<String>, explicit_flag: &bool) -> Vec<EntryData>{
    if keyword.is_none() {
        eprintln!("no keywords given!");
    }

    let mut entries = vec![];
    for node in &database.root {
        match node {
            NodeRef::Entry(entry) => {
                let entry_title = entry.get_title().unwrap();

                if !explicit_flag {
                    if !string_match::match_entry_names(entry_title, keyword.as_ref().unwrap()) {
                        continue;
                    }
                } else {
                    if entry_title != keyword.as_ref().unwrap() {
                        continue;
                    }
                }

                entries.push(EntryData::new(
                    entry_title,
                    entry.get_username().unwrap_or("No Username"),
                    entry.get_password().unwrap_or("No Password"),
                ));
            },
            _ => {}
        }
    }

    entries
}

fn prompt_for_entry() -> Result<usize, std::num::ParseIntError> {
    println!("Multiple Entries matched your query. Please select one:");
    let mut selected_entry = String::new();
    let _ = std::io::stdin().read_line(&mut selected_entry);
    selected_entry.trim().parse::<usize>()
}

fn select_entry_from_list(entry_list: &Vec<EntryData>) -> usize {
    let mut i = 1;

    for entry in entry_list {
        println!("[{}]: {}", i, entry.title());
        i += 1;
    }

    match prompt_for_entry() {
        Ok(result) => result,
        Err(_) => 1,
    }
}

pub fn output_selection(entry_list: Vec<EntryData>, clipboard_context: &mut ClipboardContext) -> bool {
    if entry_list.is_empty() {
        return false;
    }

    if entry_list.len() == 1 {
        entry_list[0].output(clipboard_context);
        return true ;
    }
    
    let selection = select_entry_from_list(&entry_list);

    entry_list[(selection - 1) as usize].output(clipboard_context);

    true
}
