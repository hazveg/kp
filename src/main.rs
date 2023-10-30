mod file;
mod db_interaction;

use db_interaction::Action;

use keepass::DatabaseKey;
use clap::Parser;

#[derive(Default, Parser, Debug)]
#[clap(author="hazveg", version="0.2.0", about)]
/// A simple KeePass database reader
pub struct Arguments {
    /// The path to the database file
    path: String,
    /// The action to be performed {list, select}
    action: Option<String>,
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
        Action::Select => db_interaction::select_entries(&database, args.keyword),
    }
}
