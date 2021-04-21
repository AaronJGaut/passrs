pub mod crypto;
pub mod file;

use std::io;
use std::io::ErrorKind::NotFound;
use crate::cli;
use crate::error::PassError;


const FILE_VERSION: u64 = 4;


pub enum DbEncryption {
    Fernet,
}

pub struct Database {
    pub filepath: String,
    pub modified: bool,
    pub data: Option<DbData>,
}

pub struct DbData {
    pub settings: DbSettings,
    records: Vec<Record>,
    pub password: Vec<u8>,
}

impl Database {
    pub fn new(filepath: &str) -> Database {
        Database {
            filepath: String::from(filepath),
            modified: false,
            data: None,
        }
    }
    pub fn require_loaded(&mut self) -> Result<&mut Database, PassError> {
        if let None = self.data {
            if let Err(err) = self.load() {
                match err {
                    PassError::NotFound => {
                        println!("No database found. Creating new database.");
                        self.create()?;
                        println!("Database created!");
                    }
                    _ => return Err(err),
                }
            }
        }
        assert!(self.data.is_some());
        Ok(self)
    }
    pub fn get_records(&mut self) -> Result<&Vec<Record>, PassError> {
        Ok(&self.require_loaded()?.data.as_ref().unwrap().records)
    }
    pub fn add_record(&mut self, record: Record) -> Result<(), PassError>{
        let records = &mut self.require_loaded()?.data.as_mut().unwrap().records;
        records.push(record);
        self.modified = true;
        Ok(())
    }
    pub fn update_record(&mut self, index: usize, record: Record) -> Result<(), PassError> {
        let records = &mut self.require_loaded()?.data.as_mut().unwrap().records;
        records[index] = record;
        self.modified = true;
        Ok(())
    }
    pub fn remove_record(&mut self, index: usize) -> Result<(), PassError> {
        let records = &mut self.require_loaded()?.data.as_mut().unwrap().records;
        records.remove(index);
        self.modified = true;
        Ok(())
    }
    fn load(&mut self) -> Result<(), PassError> {
        println!("Loading database from \"{}\"", self.filepath);
        self.data = Some(file::load(&self.filepath)?);
        println!("Database loaded!");
        Ok(())
    }
    pub fn save(&mut self, filepath: Option<&str>) -> Result<(), PassError> {
        self.require_loaded()?;
        let filepath = match filepath {
            Some(path) => path,
            None => &self.filepath,
        };
        println!("Saving database to \"{}\"", filepath);
        file::save(filepath, self.data.as_ref().unwrap())?;
        println!("Database saved!");
        self.modified = false;
        Ok(())
    }
    fn create(&mut self) -> Result<(), PassError> {
        let password = cli::create_password(
            "Choose a master password: ",
            "Repeat to confirm: ",
            "Mismatch. Please try again: ",
        )?.as_bytes().to_vec();
        self.data = Some(DbData {
            settings: DbSettings {},
            records: Vec::new(),
            password: password,
        });
        self.modified = true;
        Ok(())
    }
    pub fn loaded(&self) -> bool {
        self.data.is_some()
    }
}

pub struct DbSettings {
}

#[derive(Clone)]
pub struct Record {
    pub account: String,
    pub username: Option<String>,
    pub password: String,
    pub notes: Option<String>,
}

pub fn parse_index(db: &mut Database, input: &str) -> Result<usize, PassError> {
    let num = usize::from_str_radix(input, 10);
    let input = String::from(input).to_ascii_lowercase();
    let records = db.get_records()?;
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
        for item in matches.iter().enumerate() {
            let record = &(item.1).1;
            println!("{:>5} {}", item.0, record.account);
            if let Some(username) = &record.username {
                println!("\tUsername: {}", username);
            }
            if let Some(notes) = &record.notes {
                println!("\tNotes:");
                for line in notes.lines() {
                    println!("\t\t{}", line);
                }
            }
        }
        let index = cli::read("Ambiguous account name. Pick a number: ", false)?;
        let num = usize::from_str_radix(&index, 10);
        if let Ok(idx) = num {
            if idx < matches.len() {
                return Ok(matches[idx].0);
            } else {
                return Err(PassError::Other(String::from("Index out of bounds")));
            }
        } else {
            return Err(PassError::Other(String::from("Not a valid index")));
        }
    }
    if let Ok(_) = num {
        return Err(PassError::Other(String::from("Index out of bounds")));
    } else {
        return Err(PassError::Other(String::from("No matching accounts found")));
    }
}
