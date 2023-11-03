#[derive(Debug, PartialEq)]
pub struct Arguments {
    // path to database file
    pub path: String,
    pub action: String,
    pub explicit: bool,
    pub keyword: Option<String>,
    pub password: Option<String>,
}

impl Arguments {
    pub fn parse(args: Vec<String>) -> Option<Arguments> {
        let mut explicit = false;
        let mut password = None;
        let mut action = "".to_string();
        let mut keyword = None;
        let mut path = "".to_string();
        
        for i in 0..args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    Self::help();
                    return None;
                },
                "-v" | "--version" => {
                    Self::version();
                    return None;
                },
                "-e" | "--explicit" => explicit = true,
                "-p" | "--password" => {
                    if (i + 1) <= args.len() {
                        password = Some(args[i + 1].clone())
                    }
                }
                "select" | "list" => action = args[i].clone(),
                _ => {
                    if args[i].len() > 5 && &args[i][(args[i].len() - 5)..] == ".kdbx" {
                        path = args[i].clone();
                        continue;
                    }
                    
                    if action == "select" {
                        keyword = Some(args[i].clone());
                    }
                },
            }
        }

        if keyword.is_none() && action == "select" {
            eprintln!("No keyword provided!");
            return None;
        }

        if action == "" {
            action = "list".to_string();
        }

        Some(Arguments {
            path,
            action,
            explicit,
            keyword,
            password,
        })
    }

    fn help() {
        println!(
            "kp\n
            A simple KeePass database reader\n"
        );

        println!("Usage: kp <PATH> [ACTION] [KEYWORD] [OPTIONS]\n");

        println!(
            "Arguments:\n
            \t<PATH>\t\tThe path to the database file [.kdbx]\n
            \t[ACTION]\t\tThe action to be performed (list, select) (defaults to list when not passed)\n
            \t[KEYWORD]\t\tThe keyword that shall be searched for (if action is 'select')\n"
        );

        println!(
            "Options:\n
            \t-e, --explicit\t\tExplicitly search for the given keyword instead of fuzzy matching\n
            \t-h, --help\t\tPrint help\n
            \t-v, --version\t\tPrint version"
        );
    }

    fn version() {
        println!("kp 0.3.0");
    }
}

#[cfg(test)]
mod tests {
    use crate::argument_parsing::Arguments;

    #[test]
    fn args() {
        let args = vec![
            String::from("doodle"),
            String::from("Database.kdbx"),
            String::from("--explicit"),
            String::from("-p"),
            String::from("fuck"),
            String::from("select"),
            String::from("Kaschuso"),
        ];

        assert_eq!(
            Some(Arguments {
                path: "Database.kdbx".to_string(),
                action: "select".to_string(),
                keyword: Some("Kaschuso".to_string()),
                explicit: true,
                password: Some("fuck".to_string()),
            }),
            Arguments::parse(args),
        );
    }
}
