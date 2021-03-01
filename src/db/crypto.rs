use password_hash::{Ident, Salt, PasswordHasher};

const PBKDF2_SHA256: Ident = Ident::new("pbkdf2-sha256");

pub fn kdf(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let salt = base64::encode(salt);
    let key = pbkdf2::Pbkdf2.hash_password(
        password,
        Some(PBKDF2_SHA256),
        None,
        pbkdf2::Params {
            rounds: iterations,
            output_length: 32,
        },
        Salt::new(&salt).unwrap(),
    ).unwrap();
    key.hash.unwrap().as_bytes().to_vec()
}

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let key = base64::encode_config(key, base64::URL_SAFE);
    let fernet = fernet::Fernet::new(&key).unwrap();
    let cyphertext = fernet.encrypt(&plaintext);
    let cyphertext = base64::decode_config(cyphertext, base64::URL_SAFE).unwrap();
    cyphertext.to_vec()
}

pub fn decrypt(cyphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, fernet::DecryptionError> {
    let key = base64::encode_config(key, base64::URL_SAFE);
    let cyphertext = base64::encode_config(cyphertext, base64::URL_SAFE);
    let fernet = fernet::Fernet::new(&key).unwrap();
    Ok(fernet.decrypt(&cyphertext)?)
}
