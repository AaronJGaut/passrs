use clap::App;

use cmd::{CommandWrapper, CommandVec};

pub mod cmd;
pub mod cli;

pub enum DbEncryption {
    Fernet,
}

pub enum DbFormat {
    Json,
}

pub struct Database {
    encryption: DbEncryption,
    format: DbFormat,
    path: String,
    records: Option<Vec<Record>>,  // Lazy load only when needed
}

pub struct Record {
    account: String,
    username: Option<String>,
    password: String,
    notes: Option<String>,
}
