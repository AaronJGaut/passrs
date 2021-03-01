pub mod crypto;
pub mod file;

use std::io;
use std::io::ErrorKind::NotFound;
use crate::cli;


const FILE_VERSION: u64 = 4;


pub enum DbEncryption {
    Fernet,
}

pub struct Database {
    filepath: String,
    modified: bool,
    data: Option<DbData>,
}

pub struct DbData {
    settings: DbSettings,
    records: Vec<Record>,
    password: Vec<u8>,
}

impl Database {
    pub fn new(filepath: &str) -> Database {
        Database {
            filepath: String::from(filepath),
            modified: false,
            data: None,
        }
    }
    pub fn require_loaded(&mut self) -> &mut Database {
        if let None = self.data {
            if let Err(err) = self.load() {
                match err.kind() {
                    NotFound => {
                        println!("No database found. Creating new database.");
                        self.create();
                        println!("Database created!");
                    }
                    _ => panic!(),
                }
            }
        }
        assert!(self.data.is_some());
        self
    }
    pub fn get_records(&mut self) -> &Vec<Record> {
        &self.require_loaded().data.as_ref().unwrap().records
    }
    pub fn add_record(&mut self, record: Record) {
        let records = &mut self.require_loaded().data.as_mut().unwrap().records;
        records.push(record);
        self.modified = true;
    }
    fn load(&mut self) -> Result<(), io::Error> {
        println!("Loading database from \"{}\"", self.filepath);
        self.data = Some(file::load(&self.filepath)?);
        println!("Database loaded!");
        Ok(())
    }
    pub fn save(&mut self, filepath: Option<&str>) -> Result<(), io::Error> {
        self.require_loaded();
        let filepath = match filepath {
            Some(path) => path,
            None => &self.filepath,
        };
        println!("Saving database to \"{}\"", filepath);
        file::save(filepath, self.data.as_ref().unwrap()).unwrap();
        println!("Database saved!");
        self.modified = false;
        Ok(())
    }
    fn create(&mut self) {
        let password = cli::create_password(
            "Choose a master password: ",
            "Repeat to confirm: ",
            "Mismatch. Please try again: ",
        ).unwrap().as_bytes().to_vec();
        self.data = Some(DbData {
            settings: DbSettings {},
            records: Vec::new(),
            password: password,
        });
        self.modified = true;
    }
}

pub struct DbSettings {
}

pub struct Record {
    pub account: String,
    pub username: Option<String>,
    pub password: String,
    pub notes: Option<String>,
}

pub fn parse_index(db: &mut Database, input: &str) -> Result<usize, String> {
    let num = usize::from_str_radix(input, 10);
    let input = String::from(input).to_ascii_lowercase();
    let records = db.get_records();
    if let Ok(idx) = num {
        if idx < records.len() {
            return Ok(idx);
        }
    }
    let mut matches = Vec::<(usize, &Record)>::new();
    for item in records.iter().enumerate() {
        let name = item.1.account.clone().to_ascii_lowercase();
        if let Some(_) = name.find(&input) {
            matches.push((item.0, &item.1))
        }
    }
    if matches.len() == 1 {
        return Ok(matches[0].0);
    }
    if matches.len() > 1 {
        return Err(String::from("Ambiguous account - more than one match"));
    }
    if let Ok(_) = num {
        return Err(String::from("Index out of bounds"));
    } else {
        return Err(String::from("No matching accounts found"));
    }
}
