use crate::Action;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub action: Action,
    pub resource: Option<String>,
}

pub struct TaskError {
    action: String,
    // We will return invalid data with this down the line
    resource: String,
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.resource.as_str() {
            "" => write!(f, "No resource provided for `{}`", self.action),
            _ => write!(f, "Invalid resource `{}` provided for `{}`", self.resource, self.action),
        }
    }
}

impl std::fmt::Debug for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ action: {}, resource: {} }}", self.action, self.resource)
    }
}

fn display_help() {
    println!("kp ACTION [RESOURCE] [ARGUMENTS...]");
    println!();
    println!("A͟c͟t͟i͟o͟n͟s͟:͟");
    println!("  ls\t\t\tList all entrys and groups in configured database");
    println!("  grep [SEARCHTERM]\tSearch for given searchterm in configured database");
    println!("  add [(GROUP/)ENTRY]\tAdd a new Entry with given name and optional group, will create group if non-existent");
    println!("  show [(GROUP/)ENTRY]\tDisplay Information of a given entry");
    println!();
    println!("A͟r͟g͟u͟m͟e͟n͟t͟s͟:");
    println!("  -h, --help\t\tDisplay this help information");
}

//These functions are currently placeholders for actual functionality, i wanna get the
//error-handling down first.
//I don't expect there to be any errors with this
fn list() {
    println!("TODO: Create listing functionality");
}

fn grep(searchterm: Option<String>) -> Result<(), TaskError> {
    if searchterm.is_none() {
        return Err(TaskError { action: "grep".to_string(), resource: "".to_string() });
    }

    println!("TODO: Create grep functionality");

    Ok(())
}

fn add(entry: Option<String>) -> Result<(), TaskError> {
    if entry.is_none() {
        return Err(TaskError { action: "add".to_string(), resource: "".to_string() });
    }

    println!("TODO: Create entry functionality");

    Ok(())
}

fn show(entry: Option<String>) -> Result<(), TaskError> {
    if entry.is_none() {
        return Err(TaskError { action: "show".to_string(), resource: "".to_string() });
    }

    println!("TODO: Create show functionality");

    Ok(())
}

pub fn perform_task(task: Task) -> Result<(), TaskError> {
    match task.action {
        Action::DisplayHelp => display_help(),
        Action::List => list(),
        Action::Grep => grep(task.resource)?,
        Action::Add => add(task.resource)?,
        Action::Show => show(task.resource)?,
    }

    Ok(())
}
