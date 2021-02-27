use std::{fs, io};
use std::io::ErrorKind::NotFound;
use crate::cli;
use password_hash::{Ident, Salt, PasswordHasher};

const FILE_VERSION: u64 = 4;

const HASH_ALGORITHM: Ident = Ident::new("pbkdf2-sha256");

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
        let f = fs::File::open(self.filepath.as_str())?;
        let reader = io::BufReader::new(f);
        let obj: serde_json::Value = serde_json::from_reader(reader).unwrap();
        let map = match obj {
            serde_json::Value::Object(map) => map,
            _ => panic!(),
        };
        let version = match &map["version"] {
            serde_json::Value::Number(num) => num.as_u64().unwrap(),
            serde_json::Value::String(num) => u64::from_str_radix(num.as_str(), 10).unwrap(),
            _ => panic!(),
        };
        if version < 3 {
            panic!();
        }
        let mut payload = String::new();
        match &map["payload"] {
            serde_json::Value::String(pl) => payload = String::from(pl.trim()),
            serde_json::Value::Array(parts) => for part in parts {
                match part {
                    serde_json::Value::String(val) => payload += val.trim(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        };
        let mut salt = String::new();
        match &map["salt"] {
            serde_json::Value::String(val) => salt = String::from(val.trim()),
            serde_json::Value::Array(parts) => for part in parts {
                match part {
                    serde_json::Value::String(val) => salt += val.trim(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        };
        let salt = base64::decode(salt).unwrap();
        let mut password: Vec<u8> = cli::read_hidden("Master password: ", false).unwrap().as_bytes().to_vec();
        let payload_bytes = base64::decode(payload).unwrap();
        let mut text = String::from("");
        loop {
            let result = decrypt(&payload_bytes, &salt, password.as_slice());
            if let Ok(result) = result {
                text = result;
                break;
            }
            password = cli::read_hidden("Wrong password. Try again: ", false).unwrap().as_bytes().to_vec();
        }
        self.data = Some(DbData {
            settings: DbSettings {},
            records: parse_records(&text),
            password: password,
        });
        println!("Database loaded!");
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

fn decrypt(cyphertext: &[u8], salt: &[u8], password: &[u8]) -> Result<String, fernet::DecryptionError> {
    let salt = base64::encode(salt);
    let key = pbkdf2::Pbkdf2.hash_password(
        password,
        Some(HASH_ALGORITHM),
        None,
        pbkdf2::Params {
            rounds: 1000000,
            output_length: 32,
        },
        Salt::new(&salt).unwrap(),
    ).unwrap();
    let hash = key.hash.unwrap();
    let hash_b64 = base64::encode_config(hash.as_bytes(), base64::URL_SAFE);
    let fernet = fernet::Fernet::new(&hash_b64).unwrap();
    let cyphertext = base64::encode_config(cyphertext, base64::URL_SAFE);
    Ok(String::from_utf8(fernet.decrypt(&cyphertext)?).unwrap())
}

fn parse_records(payload_text: &str) -> Vec<Record> {
    let obj: serde_json::Value = serde_json::from_str(payload_text).unwrap();
    let map = match obj {
        serde_json::Value::Object(map) => map,
        _ => panic!(),
    };
    let records_json = match &map["records"] {
        serde_json::Value::Array(array) => array,
        _ => panic!(),
    };
    let mut records = Vec::<Record>::new();
    for record_json in records_json {
        let record_map = match &record_json {
            serde_json::Value::Object(record_map) => record_map,
            _ => panic!(),
        };
        let account = match record_map.get("account").unwrap() {
            serde_json::Value::String(entry) => entry.clone(),
            _ => panic!(),
        };
        let username = match record_map.get("username") {
            None => None,
            Some(val) => match val {
                serde_json::Value::String(entry) => Some(entry.clone()),
                _ => panic!(),
            },
        };
        let password = match record_map.get("password").unwrap() {
            serde_json::Value::String(entry) => entry.clone(),
            _ => panic!(),
        };
        let notes = match record_map.get("notes") {
            None => None,
            Some(val) => match val {
                serde_json::Value::String(entry) => Some(entry.clone()),
                _ => panic!(),
            }
        };
        records.push(Record {account: account, username: username, password: password, notes: notes});
    }
    records
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
