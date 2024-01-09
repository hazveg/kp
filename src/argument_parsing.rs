use crate::{Task, Action};

pub struct ArgParseError {
    resolved_action: bool,
    point_of_contension: String,
}

impl std::fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.resolved_action, &self.point_of_contension[..]) {
            (false, "") => write!(f, "No action provided"),
            (false, _) => write!(f, "Invalid action `{}` provided", self.point_of_contension),
            (true, "") => write!(f, "No resource provided"),
            (_, _) => write!(f, "this shouldn't be able to pop up"),
        }
    }
}

impl std::fmt::Debug for ArgParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ resolved_action: {}, point_of_contension: {} }}", self.resolved_action, self.point_of_contension)
    }
}

pub fn parse_arguments(args: std::env::Args) -> Result<Task, ArgParseError> {
    let args: Vec<String> = args.collect();

    let action = match args.get(1) {
        None => return Err(ArgParseError { resolved_action: false, point_of_contension: "".to_string() }),
        Some(input) => match input.to_lowercase().as_str() {
            "ls" | "l" => Action::List,
            "grep" | "g" => Action::Grep,
            "add" | "a" => Action::Add,
            "show" | "s" => Action::Show,
            _ => return Err(ArgParseError { resolved_action: false, point_of_contension: input.to_string() }),
        }
    };

    if args.get(2) == None && action != Action::List {
        return Err(ArgParseError { resolved_action: true, point_of_contension: "".to_string() });
    }

    Ok(Task { action, resource: args.get(2).cloned() })
}
