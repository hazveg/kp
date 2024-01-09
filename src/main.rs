mod argument_parsing;

#[derive(PartialEq, Debug)]
pub enum Action {
    List,
    Grep,
    Add,
    Show,
}

#[derive(Debug)]
pub struct Task {
    action: Action,
    resource: Option<String>,
}

fn main() {
    let task = match argument_parsing::parse_arguments(std::env::args()) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(task) => task,
    };

    dbg!(task);
}
