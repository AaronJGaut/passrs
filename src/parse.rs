use crate::db;
use std::str::FromStr;

pub trait Parse: Sized {
    fn parse(s: &str, db: &mut db::Database) -> Result<Self, String>;
}

impl<T: FromStr> Parse for T {
    fn parse(s: &str, db: &mut db::Database) -> Result<T, String> {
        match T::from_str(s) {
            Ok(t) => Ok(t),
            Err(_) => Err(String::from("Failed to parse")),
        }
    }
}
