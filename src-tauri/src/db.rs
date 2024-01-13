// Sled DB functions
use crate::wallet::{Wall, initialize_wallet_strings};
use anyhow::anyhow;
use bdk::KeychainKind;
use orion::{pwhash, kdf, util, aead};
use sled::Db;
use tauri::api::path;


fn sled_db() -> Db {
    let data_dir_path = path::home_dir().unwrap(); // TODO: change db dir
    let db_path = data_dir_path.join("wallets_info.db");
    let db: sled::Db = sled::open(&db_path).unwrap();
    db
}

pub fn save_wallet(
    name: &String,
    password: &String,
    wallet: &Wall,
) -> anyhow::Result<()> {
    // async?
    let db = sled_db();
    let tree = db.open_tree(name)?;

    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let passw_hash = pwhash::hash_password(&passw, 3, 1 << 16)?;

    // TODO: proper salt
    let mut salt_bytes = [0u8; 64];
    util::secure_rand_bytes(&mut salt_bytes)?;
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;

    let desc = wallet
        .get_descriptor_for_keychain(KeychainKind::External)
        .to_string();
    let change_desc = wallet
        .get_descriptor_for_keychain(KeychainKind::Internal)
        .to_string();

    tree.insert(b"descriptor", aead::seal(&derived_key, desc.as_bytes())?)?;
    tree.insert(
        b"change_descriptor",
        aead::seal(&derived_key, change_desc.as_bytes())?,
    )?;
    tree.insert(b"password_hash", passw_hash.unprotected_as_encoded())?;
    tree.insert(b"enc_salt", salt_bytes.to_vec())?;

    Ok(())
}

pub fn retrieve_wallet(name: &String, password: &String) -> anyhow::Result<Wall> {
    let db = sled_db();
    let tree = db.open_tree(name)?;

    // TODO: passwd comparison
    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let stored_hash = tree.get(b"password_hash")?.unwrap();
    let hash = pwhash::PasswordHash::from_encoded(std::str::from_utf8(&stored_hash)?)?;
    if !pwhash::hash_password_verify(&hash, &passw).is_ok() {
        return Err(anyhow!("Wrong password"));
    }

    let salt_bytes = tree.get(b"enc_salt")?.unwrap().to_vec();
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;

    let enc_desc = tree.get(b"descriptor")?.unwrap();
    let desc = aead::open(&derived_key, &enc_desc)?;
    let enc_change_desc = tree.get(b"change_descriptor")?.unwrap();
    let change_desc = aead::open(&derived_key, &enc_change_desc)?;

    let wallet = initialize_wallet_strings(
        name,
        &String::from_utf8(desc).unwrap(),
        &String::from_utf8(change_desc).unwrap(),
    );
    println!("wallet is probably OK retrieved");
    Ok(wallet)
}
