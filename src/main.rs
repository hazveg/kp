mod argument_parsing;
mod file;
mod db_interaction;
mod string_match;

use keepass::DatabaseKey;

fn main() {
    let env_args: Vec<String> = std::env::args().collect();

    let args = match argument_parsing::Arguments::parse(env_args) {
        Some(args) => args,
        None => return,
    };
    
    let password = file::set_password(&args);
    let database = file::unlock_database(&args.path, DatabaseKey::with_password(&password));

    match args.action.as_str() {
        "list" => db_interaction::list_entries(&database),
        "select" => {
            let entry_list = db_interaction::select_entries(&database, &args.keyword, &args.explicit);

            if !db_interaction::output_selection(entry_list) {
                eprintln!("No entries pertaining to `{}` were found", &args.keyword.unwrap());
            }
        },
        _ => {},
    }
}
