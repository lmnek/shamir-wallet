// Sled DB functions

use crate::wallet::{initialize_wallet_strings, MyWallet};
use anyhow::anyhow;
use orion::{aead, kdf, pwhash, util};
use sled::Db;
use tauri::api::path;

fn sled_db() -> Db {
    let data_dir_path = path::home_dir().unwrap(); // TODO: change db dir
    let db_path = data_dir_path.join("wallets_info.db");
    sled::open(&db_path).unwrap()
}

// PERF: async / look at bottlenecks
pub fn save_wallet(name: &String, password: &String, wallet: &MyWallet) -> anyhow::Result<()> {
    let db = sled_db();
    let tree = db.open_tree(name)?;

    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let passw_hash = pwhash::hash_password(&passw, 3, 1 << 16)?;
    tree.insert(b"password_hash", passw_hash.unprotected_as_encoded())?;

    let (desc, change_desc) = wallet.get_descritors();

    let mut salt_bytes = [0u8; 64];
    util::secure_rand_bytes(&mut salt_bytes)?;
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;

    tree.insert(b"enc_salt", salt_bytes.to_vec())?;
    tree.insert(b"descriptor", aead::seal(&derived_key, desc.as_bytes())?)?;
    tree.insert(
        b"change_descriptor",
        aead::seal(&derived_key, change_desc.as_bytes())?,
    )?;

    Ok(())
}

pub fn retrieve_wallet(name: &String, password: &String) -> anyhow::Result<MyWallet> {
    let db = sled_db();
    let tree = db.open_tree(name)?;

    // verify password with hash in DB
    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let stored_hash = tree.get(b"password_hash")?.unwrap();
    let hash = pwhash::PasswordHash::from_encoded(std::str::from_utf8(&stored_hash)?)?;
    if !pwhash::hash_password_verify(&hash, &passw).is_ok() {
        return Err(anyhow!("Wrong password"));
    }

    // derive decryption key from password
    let salt_bytes = tree.get(b"enc_salt")?.unwrap().to_vec();
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;

    // retrive and decrypt descriptors
    let enc_desc = tree.get(b"descriptor")?.unwrap();
    let desc = aead::open(&derived_key, &enc_desc)?;
    let enc_change_desc = tree.get(b"change_descriptor")?.unwrap();
    let change_desc = aead::open(&derived_key, &enc_change_desc)?;

    // construct wallet
    let wallet = initialize_wallet_strings(
        name,
        &String::from_utf8(desc).unwrap(),
        &String::from_utf8(change_desc).unwrap(),
    );
    Ok(wallet)
}
