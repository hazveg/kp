mod argument_parsing;
mod task;

use task::Task;

#[derive(PartialEq, Debug)]
pub enum Action {
    List,
    Grep,
    Add,
    Show,
    DisplayHelp
}

fn main() {
    let task = match argument_parsing::parse_arguments(std::env::args()) {
        Err(e) => {
            eprintln!("kp: {}\nPlease refer to \"kp --help\".", e);
            std::process::exit(1);
        },
        Ok(task) => task,
    };

    match task::perform_task(task) {
        Err(e) => {
            eprintln!("kp: {}\nPlease input a valid value.", e);
            std::process::exit(2);
        },
        Ok(()) => {},
    }
}
