mod file;
mod db_interaction;
mod string_match;

use db_interaction::Action;

use keepass::DatabaseKey;
use clap::Parser;
use arboard::Clipboard;

#[derive(Default, Parser, Debug)]
#[clap(author="hazveg", version="0.2.0", about)]
/// A simple KeePass database reader
pub struct Arguments {
    /// The path to the database file
    path: String,
    /// The action to be performed {list, select}
    action: Option<String>,
    /// Explicitly search for the given keyword instead of fuzzy matching
    #[arg(default_value_t=false, short, long)]
    explicit: bool,
    /// The keyword that shall be searched for (if action is 'select')
    keyword: Option<String>,
    /// The password to unlock the database
    #[arg(short, long)]
    password: Option<String>,
}

fn main() {
    let args = Arguments::parse();
    
    let password = file::set_password(&args);
    let database = file::unlock_database(args.path, DatabaseKey::with_password(&password));

    let action = match args.action {
        Some(action) => db_interaction::define_action(&action[..]),
        None => Action::List
    };

    match action {
        Action::List => db_interaction::list_entries(&database),
        Action::Select => {
            let mut clipboard = Clipboard::new().unwrap();
            let entry_list = db_interaction::select_entries(&database, &args.keyword, &args.explicit);

            if !db_interaction::output_selection(entry_list, &mut clipboard) {
                eprintln!("No entries pertaining to `{}` were found", &args.keyword.unwrap());
            }
        },
    }
}
