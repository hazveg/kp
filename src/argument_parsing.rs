use crate::{Task, Action};

pub struct ArgParseError {
    point_of_contension: String,
}

impl std::fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.point_of_contension.as_str() {
            "" => write!(f, "No action provided"),
            _ => write!(f, "Invalid action `{}` provided", self.point_of_contension),
        }
    }
}

impl std::fmt::Debug for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ point_of_contension: {} }}", self.point_of_contension)
    }
}

pub fn parse_arguments(args: std::env::Args) -> Result<Task, ArgParseError> {
    let args: Vec<String> = args.collect();

    let action = match args.get(1) {
        None => return Err(ArgParseError { point_of_contension: "".to_string() }),
        Some(input) => match input.to_lowercase().as_str() {
            "ls" | "l" => Action::List,
            "grep" | "g" => Action::Grep,
            "add" | "a" => Action::Add,
            "show" | "s" => Action::Show,
            "--help" | "-h" => Action::DisplayHelp,
            _ => return Err(ArgParseError { point_of_contension: input.to_string() }),
        }
    };

    Ok(Task { action, resource: args.get(2).cloned() })
}
