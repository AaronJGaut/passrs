use std::fs;
use std::io::ErrorKind::NotFound;

pub enum DbEncryption {
    Fernet,
}

pub struct Database {
    filepath: String,
    settings: Option<DbSettings>, // Lazy load only when needed
    records: Option<Vec<Record>>, // Lazy load only when needed
}

impl Database {
    pub fn new(filepath: &str) -> Database {
        Database {
            filepath: String::from(filepath),
            settings: None,
            records: None,
        }
    }
    pub fn require_loaded(&mut self) -> &mut Database {
        if let None = self.records {
            self.load();
        }
        self
    }
    pub fn get_records(&mut self) -> &Vec<Record> {
        self.require_loaded().records.as_ref().unwrap()
    }
    pub fn add_record(&mut self, record: Record) {
        let records = self.require_loaded().records.as_mut().unwrap();
        records.push(record);
    }
    fn load(&mut self) {
        match fs::File::open(self.filepath.as_str()) {
            Ok(f) => {
                println!("File found. Loading not yet implemented");
            }
            Err(err) => match err.kind() {
                NotFound => {
                    println!("File not found. Creating new database");
                    self.create();
                }
                _ => panic!(),
            },
        }
    }
    fn create(&mut self) {
        self.settings = Some(DbSettings {
            encryption: DbEncryption::Fernet,
        });
        self.records = Some(Vec::new());
    }
}

pub struct DbSettings {
    encryption: DbEncryption,
}

pub struct Record {
    pub account: String,
    pub username: Option<String>,
    pub password: String,
    pub notes: Option<String>,
}
