pub enum DbEncryption {
    Fernet,
}

pub enum DbFormat {
    Json,
}

pub struct Database {
    filepath: String,
    settings: Option<DbSettings>,  // Lazy load only when needed
    records: Option<Vec<Record>>,  // Lazy load only when needed
}

impl Database {
    pub fn new(filepath: &str) -> Database {
        Database {filepath : String::from(filepath), settings : None, records : None}
    }
}

pub struct DbSettings {
    encryption: DbEncryption,
    format: DbFormat,
}

pub struct Record {
    account: String,
    username: Option<String>,
    password: String,
    notes: Option<String>,
}
