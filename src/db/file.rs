use std::{fs, io};
use crate::cli;
use crate::db::{self, crypto};
use rand::Rng;

const VERSION: &'static str = "4";

pub fn load(filepath: &str) -> Result<db::DbData, io::Error> {
    let version = parse_version(filepath)?;
    match version {
        4 => load_v4(filepath),
        3 => load_v3(filepath),
        _ => panic!(),
    }
}

pub fn save(filepath: &str, data: &db::DbData) -> Result<(), io::Error> {
    let mut records_array = Vec::<serde_json::Value>::new();
    for record in &data.records {
        let mut record_json = serde_json::Map::new();
        record_json.insert(String::from("account"), serde_json::Value::String(record.account.clone()));
        if let Some(username) = &record.username {
            record_json.insert(String::from("username"), serde_json::Value::String(username.clone()));
        }
        record_json.insert(String::from("password"), serde_json::Value::String(record.password.clone()));
        if let Some(notes) = &record.notes {
            record_json.insert(String::from("notes"), serde_json::Value::String(notes.clone()));
        }
        records_array.push(serde_json::Value::Object(record_json));
    }
    let payload_json = serde_json::json!({
        "records": records_array,
    });
    let payload = payload_json.to_string();
    let payload_bytes = payload.as_bytes();
    let mut salt = [0u8; 18];
    rand::thread_rng().fill(&mut salt);
    let key = crypto::kdf(data.password.as_slice(), &salt, 1000000);
    let payload = crypto::encrypt(&payload_bytes, &key);
    let payload = base64::encode(&payload);
    let mut payload_array = Vec::<serde_json::Value>::new();
    for chunk in payload.as_bytes().chunks(50) {
        payload_array.push(serde_json::Value::String(String::from_utf8(chunk.to_vec()).unwrap()));
    }
    let json = serde_json::json!({
        "Version": VERSION,
        "Salt": base64::encode(&salt),
        "payload": payload_array,
    });
    let tmp_filepath = String::from(filepath) + ".part";
    {
        let f = fs::File::create(&tmp_filepath)?;
        let writer = io::BufWriter::new(f);
        serde_json::to_writer_pretty(writer, &json)?;
    }
    fs::rename(tmp_filepath, filepath);

    Ok(())
}

fn parse_version(filepath: &str) -> Result<u64, io::Error> {
    let f = fs::File::open(filepath)?;
    let reader = io::BufReader::new(f);
    let obj: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let map = match obj {
        serde_json::Value::Object(map) => map,
        _ => panic!(),
    };
    let version_json = match map.get("Version") {
        Some(val) => val,
        None => match map.get("version") {
            Some(val) => val,
            None => panic!(),
        },
    };

    let version = match &version_json {
        serde_json::Value::Number(num) => num.as_u64().unwrap(),
        serde_json::Value::String(num) => u64::from_str_radix(num.as_str(), 10).unwrap(),
        _ => panic!(),
    };
    Ok(version)
}

fn load_v3(filepath: &str) -> Result<db::DbData, io::Error> {
    let f = fs::File::open(filepath)?;
    let reader = io::BufReader::new(f);
    let obj: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let map = match obj {
        serde_json::Value::Object(map) => map,
        _ => panic!(),
    };
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
        let key = crypto::kdf(password.as_slice(), &salt, 1000000);
        let result = crypto::decrypt(&payload_bytes, &key);
        if let Ok(result) = result {
            text = String::from_utf8(result).unwrap();
            break;
        }
        password = cli::read_hidden("Wrong password. Try again: ", false).unwrap().as_bytes().to_vec();
    }
    Ok(db::DbData {
        settings: db::DbSettings {},
        records: parse_records(&text),
        password: password,
    })
}

fn parse_records(payload_text: &str) -> Vec<db::Record> {
    let obj: serde_json::Value = serde_json::from_str(payload_text).unwrap();
    let map = match obj {
        serde_json::Value::Object(map) => map,
        _ => panic!(),
    };
    let records_json = match &map["records"] {
        serde_json::Value::Array(array) => array,
        _ => panic!(),
    };
    let mut records = Vec::<db::Record>::new();
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
                serde_json::Value::String(entry) => if entry.is_empty() {
                    None
                } else {
                    Some(entry.clone())
                },
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
                serde_json::Value::String(entry) => if entry.is_empty() {
                    None
                } else {
                    Some(entry.clone())
                },
                _ => panic!(),
            }
        };
        records.push(db::Record {account: account, username: username, password: password, notes: notes});
    }
    records
}


fn load_v4(filepath: &str) -> Result<db::DbData, io::Error> {
    let f = fs::File::open(filepath)?;
    let reader = io::BufReader::new(f);
    let obj: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let map = match obj {
        serde_json::Value::Object(map) => map,
        _ => panic!(),
    };
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
    match &map["Salt"] {
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
        let key = crypto::kdf(password.as_slice(), &salt, 1000000);
        let result = crypto::decrypt(&payload_bytes, &key);
        if let Ok(result) = result {
            text = String::from_utf8(result).unwrap();
            break;
        }
        password = cli::read_hidden("Wrong password. Try again: ", false).unwrap().as_bytes().to_vec();
    }
    Ok(db::DbData {
        settings: db::DbSettings {},
        records: parse_records(&text),
        password: password,
    })
}

