use keepass::{DatabaseKey, Database, error::{DatabaseOpenError, DatabaseKeyError}};

pub fn unlock_database(path: String, key: DatabaseKey) -> Database {
    let path = std::path::Path::new(&path);

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open given file: {}", e),
    };
    
    match Database::open(&mut file, key) {
        Ok(db) => db,
        Err(e) => {
            match e {
                DatabaseOpenError::Key(ke) => match ke {
                    DatabaseKeyError::IncorrectKey => panic!("Wrong password!"),
                    DatabaseKeyError::InvalidKeyFile => panic!("Keyfile is invalid"),
                    DatabaseKeyError::Io(io_e) => panic!("An I/O error occured: {}", io_e),
                    DatabaseKeyError::Xml(xml_e) => panic!("An XML error occured: {}", xml_e),
                    DatabaseKeyError::Cryptography(cryp_e) => panic!("A cryptographic Error occured: {}", cryp_e),
                },
                DatabaseOpenError::Io(io_e) => panic!("An I/O error occured: {}", io_e),
                DatabaseOpenError::DatabaseIntegrity(db_e) => panic!("The database is corrupted: {}", db_e),
                DatabaseOpenError::UnsupportedVersion => panic!("The database version can't be read by this program"),
            }
        }
    }
}

pub fn set_password(args: &crate::Arguments) -> String {
    if args.password == None {
        match rpassword::prompt_password("Database master password: ") {
            Ok(password) => password,
            Err(e) => {
                eprintln!("Failed to read master password! {}", e);
                std::process::exit(1);
            },
        }        
    } else {
        args.password.as_ref().unwrap().to_string()
    }
}
